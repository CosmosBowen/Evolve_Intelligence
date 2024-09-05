use std::f32::consts::PI;

use crate::creature::*;
use crate::food::*;
use crate::POPULATION_SIZE;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

use neural_network::Network;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use nalgebra as na;

pub struct World {
    pub(crate) creatures: Vec<Creature>,
    pub(crate) foods: Vec<Food>,
    pub(crate) age: i32,
    pub(crate) foods_left_num: i32,
    rng: ThreadRng,
}

impl World {

    pub fn new(width: f32, height: f32, brains_optional: Option<Vec<Network>>, foods_number: i32) -> Self {
        let mut rng = thread_rng();
        // Initialize with some creatures and food
        let creatures = if let Some(brains) = brains_optional {
            assert!(!brains.is_empty());

            brains
            .into_iter()
            .map(|brain| Creature::new(
                na::Point2::new(rng.gen::<f32>() * width, rng.gen::<f32>() * height),
                rng.gen::<f32>() * 2.0 * PI,
                (rng.gen::<f32>() * SPEED_MAX).max(SPEED_MIN),
                &mut rng,
                Some(brain),
            ))
            .collect()
        } else {
            (0..POPULATION_SIZE)
            .map(|_| Creature::new(
                na::Point2::new(rng.gen_range(0.0..=1.0) * width, rng.gen_range(0.0..=1.0) * height),
                rng.gen::<f32>() * 2.0 * PI,
                (rng.gen::<f32>() * SPEED_MAX).max(SPEED_MIN),
                &mut rng,
                None,
            ))
            .collect()
        };

        let foods = (0..foods_number)
                    .map(|_| Food::new(
                        na::Point2::new(rng.gen_range(0.05..=0.95) * width, rng.gen_range(0.05..=0.95) * height)
                    ))
                    .collect();
        
        World { creatures, foods, age:0, foods_left_num: foods_number, rng}
    }


    pub fn update(&mut self) -> bool {
        self.age += 1;
        // println!("evolution epoch: {} - world update: {}", evolution_epoch, self.age);

        for creature in &mut self.creatures {
            creature.move_for_foods(&self.foods);
            
            creature.position.x = na::wrap(creature.position.x, WINDOW_WIDTH * 0.0, WINDOW_WIDTH * 1.0);
            // creature.position.x = na::wrap(creature.position.x, WINDOW_WIDTH * 0.1, WINDOW_WIDTH * 0.9);
            creature.position.y = na::wrap(creature.position.y, WINDOW_HEIGHT * 0.0, WINDOW_HEIGHT * 1.0);
            // creature.position.y = na::wrap(creature.position.y, WINDOW_HEIGHT * 0.1, WINDOW_HEIGHT * 0.9);

            // for (i, food) in self.foods.iter_mut().enumerate() {
            for food in &mut self.foods {
                if creature.eat(food) {
                    food.position = na::Point2::new(
                        // self.rng.gen::<f32>() * WINDOW_WIDTH, 
                        // self.rng.gen::<f32>() * WINDOW_HEIGHT,
                        self.rng.gen_range(0.05..=0.95) * WINDOW_WIDTH,
                        self.rng.gen_range(0.05..=0.95) * WINDOW_HEIGHT,
                    );
                }
                // if food.is_eaten == false {
                //     if creature.eat(food) {
                //         food.is_eaten = true;
                //         self.foods_left_num -= 1;
                //         // println!("foods_left_num: {}", self.foods_left_num);
                //         // if self.foods_left_num <= 0 {
                //         //     return true;
                //         // }
                //     }
                // }
                    
            }
        }
            
        false

    }

}


