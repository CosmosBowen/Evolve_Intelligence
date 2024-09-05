use genetic_algorithm::{Chromosome, Individual};
use neural_network::{LayerTopology, Network};

use crate::{Creature, CREATURE_EYE_CELLS};

pub struct CreatureIndividual {
    chromosome: Chromosome,
    fitness: f32,
}


impl CreatureIndividual {

    pub fn from_creature(creature: &Creature) -> Self {
        Self {
            chromosome: Chromosome::from_iter(creature.brain.get_params()),
            fitness: creature.eat as f32,
        }
    }

    pub fn into_brain(&self) -> Network {
        let brain = Network::from_params(
            &[ // 5 3
                LayerTopology {num_neuron: CREATURE_EYE_CELLS}, // 13
                LayerTopology {num_neuron: 5},
                LayerTopology {num_neuron: 3},
                LayerTopology {num_neuron: 2},
            ], 
            self.chromosome.clone().into_iter(),
        );

        brain
    }
}

impl Individual for CreatureIndividual {
    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn create(chromosome: Chromosome) -> Self {
        Self {
            chromosome,
            fitness: 0.0,
        }
    }
}