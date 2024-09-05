use std::iter::once;

use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct LayerTopology {
    pub num_neuron: usize,
}

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>
}
impl Network {

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
                                .windows(2)
                                .map(|adjacent_layers| Layer::random(rng, adjacent_layers[0].num_neuron, adjacent_layers[1].num_neuron))
                                .collect();
        
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn get_params(&self) -> Vec<f32> {
        self.layers.iter()
                    .flat_map(|layer| layer.neurons.iter())
                    .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
                    .copied()
                    .collect()
    }

    pub fn from_params(
        layers: &[LayerTopology],
        params: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(layers.len() > 1);

        let mut params = params.into_iter();

        let layers = layers
            .windows(2)
            .map(|adjacent_layers| {
                    Layer::from_params(
                        adjacent_layers[0].num_neuron, 
                        adjacent_layers[1].num_neuron, 
                        &mut params,
                    )
                })
            .collect();
        
        if params.next().is_some() {
            panic!("too much weights")
        }
        
        Self { layers }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn from_params(
        input_size: usize, 
        output_size: usize, 
        params: &mut dyn Iterator<Item = f32>
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_params(input_size, params))
            .collect();
        
        Self { neurons }
    }

    fn random(rng: &mut dyn RngCore, input_size:usize, output_size:usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        
        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

// trait Activation {
//     fn activation(value: f32) -> f32;
// }

#[derive(PartialEq)]
#[derive(Debug, Clone)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron { // random::<f32>()

    fn from_params(
        input_size: usize, 
        params: &mut dyn Iterator<Item = f32>
    ) -> Self {
        let bias = params.next().expect("got not enough weights");
        let weights = (0..input_size)
                                .map(|_| params.next().expect("got not enough weights"))
                                .collect();
        Self { weights, bias }
    }

    fn random(rng: &mut dyn RngCore, input_size:usize) -> Self {
        // let mut rng = rand::thread_rng();
        let weights = (0..input_size)
                                .map(|_| rng.gen_range(-1.0..=1.0))
                                .collect();
        let bias = rng.gen_range(-1.0..=1.0);

        Self { weights, bias}
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
                        .iter()
                        .zip(self.weights.iter())
                        .map(|(input, weight)| input * weight)
                        .sum::<f32>();
        
        (output + self.bias).max(0.0)

    }
}


#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;


    #[test]
    fn from_params() {
        let layers = vec![
            LayerTopology {num_neuron: 3},
            LayerTopology {num_neuron: 2},
        ];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_params(&layers, weights.clone());
        let actual = network.get_params();
        let expected = weights;
        // assert_eq!(actual, expected);
        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }
    
    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);
        assert_relative_eq!(neuron.bias, 0.5238807);
        assert_relative_eq!(
            neuron.weights.as_slice(), 
            [-0.6255188, 0.67383957, 0.8181262, 0.26284897].as_ref()
        );
    }

    #[test]
    fn propagate_neuron() {
        // let mut rng = ChaCha8Rng::from_seed(Default::default());  
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3 , 0.8],
        };
        assert_relative_eq!(
            neuron.propagate(&[-10.0, -10.0]),
            0.0
        );

        assert_relative_eq!(
            neuron.propagate(&[0.6, 0.3]),
            (0.6 * -0.3 + 0.3 * 0.8 + 0.5f32).max(0.0)
        );
    }
    
    #[test]
    fn propagate_layer() {
        let neurons = vec![
            Neuron {
                bias: 0.1, 
                weights: vec![0.1, -0.3, 0.2]
            },
            Neuron {
                bias: -0.2,
                weights: vec![0.4, 0.1, -0.5]
            }
        ];

        let layer = Layer {
            neurons,
        };
        
        let inputs = vec![0.5_f32, -0.2, 0.1];
        let expected_outputs = [0.23_f32, 0.0];
        let actual_outputs = layer.propagate(inputs);

        assert_relative_eq!(actual_outputs.as_slice(), expected_outputs.as_ref());
    }

    #[test]
    fn propagate_network() {
        let layers = vec![
            Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.1, 
                        weights: vec![0.1, -0.3, 0.2]
                    },
                    Neuron {
                        bias: -0.2,
                        weights: vec![0.4, 0.1, -0.5]
                    },
                ]
            },
            Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.1,
                        weights: vec![0.2, -0.4]
                    },
                    Neuron {
                        bias: 0.2,
                        weights: vec![-0.3, 0.1]
                    },
                    Neuron {
                        bias: -0.1,
                        weights: vec![0.5, 0.2]
                    },
                ]
            },
            Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.1,
                        weights: vec![0.3, -0.2, 0.1]
                    }
                ]
            }
        ];
        
        let network = Network {
            layers
        };

        let inputs = vec![0.5_f32, -0.2, 0.1];
        let expected_output = [0.1191_f32];
        let actual_output = network.propagate(inputs);

        assert_relative_eq!(actual_output.as_slice(), expected_output.as_ref());
    }
}

// fn main() {
//     let layers = vec![
//         LayerTopology {num_neuron: 5},
//         LayerTopology {num_neuron: 3},
//         LayerTopology {num_neuron: 2}
//     ];

//     let network_a = Network::random(&layers);
//     let network_b = Network::random(&layers);
// }