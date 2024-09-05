use genetic_algorithm::*;
use neural_network::Network;
use crate::{world::*, creature_individual::*};

use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

const MAX_GENERATION_AGE: i32 = 3500;
pub const POPULATION_SIZE: i32 = 20;

const FOOD_NUMBER: i32 = 20;
pub struct Simulation {
    pub world: World,
    genetic_algorithm: GenericAlgorithm::<RouletteWheelSelection>,
    pub width: f32,
    pub height: f32,
    evolution_epoch: i32,
}

impl Simulation {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            world: World::new(width, height, None, FOOD_NUMBER),
            genetic_algorithm: GenericAlgorithm::new(
                RouletteWheelSelection, 
                UniformCrossover, 
                GussianMutation::new(0.01, 0.2)
            ),
            width,
            height,
            evolution_epoch: 0,

        }
    }

    pub fn update(&mut self) {
        // if self.world.update() {
        //     self.evolve();
        //     self.evolution_epoch += 1;
        // }

        if self.world.age >= MAX_GENERATION_AGE {
            self.evolve();
            self.evolution_epoch += 1;
        } else {
            self.world.update();
        }
    }

    fn get_generation_info(&self, population: &Vec<CreatureIndividual> ) {
        let mut min_eat: f32 = population[0].fitness();
        let mut max_eat: f32 = population[0].fitness();
        let mut total_eat: f32 = 0.0;
        for individual in population {
            let fitness = individual.fitness();
            min_eat = min_eat.min(fitness);
            max_eat = max_eat.max(fitness);
            total_eat += individual.fitness();
        }
        let avg_eat = total_eat / POPULATION_SIZE as f32;

        // println!("min: {}, max: {}, avg: {} - evolution: {}, world age: {}", min_eat, max_eat, avg_eat, self.evolution_epoch, self.world.age);
        println!("min: {}, max: {}, avg: {} - evolution: {}, foods left num: {}", min_eat, max_eat, avg_eat, self.evolution_epoch, self.world.foods_left_num);

    }

    fn evolve(&mut self) {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population: Vec<CreatureIndividual> = self
        .world
        .creatures
        .iter()
        .map(|creature: &crate::Creature| {
            CreatureIndividual::from_creature(creature)
        })
        .collect();

        self.get_generation_info(&population);

        let best_individual_brain = &self.world.creatures
        .iter()
        .max_by(|&a, &b| a.eat.cmp(&b.eat))
        .expect("Failed to find the best creature")
        .brain;

        let children = self.genetic_algorithm.evolve(&mut rng, &population);

        let mut brains: Vec<Network> = children
        .iter()
        .map(|child| {
            child.into_brain()
        })
        .collect();

        brains.push(best_individual_brain.clone());

        self.world = World::new(self.width, self.height, Some(brains), FOOD_NUMBER);
    }
}