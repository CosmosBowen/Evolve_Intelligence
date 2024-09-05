use rand::seq::SliceRandom;
use rand::{Rng, RngCore};


#[derive(Debug, Clone)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

use std::ops::Index;

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub struct GenericAlgorithm<S> {
    select_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GenericAlgorithm<S> 
where
    S: SelectionMethod,
{
    pub fn new(
        select_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self { 
            select_method, 
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> 
    where 
        I: Individual
    {
        assert!(!population.is_empty());

        (0..population.len() - 1)
        .map(|_| {
            let parent_a = self.select_method.select(rng, population).chromosome();
            let parent_b = self.select_method.select(rng, population).chromosome();

            let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
            
            self.mutation_method.mutate(rng, &mut child);

            I::create(child)
        })
        .collect()

    }
}

pub trait Individual{
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where 
        I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I 
    where
        I: Individual
    {
        
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("population should not be empty")
    }
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome, 
        parent_b: &Chromosome
    ) -> Chromosome;
}

pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()

    }
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct GussianMutation {
    chance: f32, // 0.0 - 1.0
    coeff: f32, // 0.0 - 3.0
}

impl GussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.gen_bool(self.chance as f64) {
                let sign = if rng.gen_bool(0.5) {1.0} else {-1.0};
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Debug, PartialEq)]
    pub enum TestIndividual {
        WithFitness {fitness: f32},
        WithChromosome {chromosome: Chromosome},
    }

    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
        }
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => {
                    chromosome.iter().sum()
                }
                Self::WithFitness { fitness } => *fitness,
            }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,

                Self::WithFitness { .. } => {
                    panic!("not supported for TestIndividual::WithFitness")
                }
            }
        }

        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }
    }

    #[test]
    fn genetic_algorithm() {

        fn individual(gene: Vec<f32>) -> TestIndividual {
            TestIndividual::create(gene.into_iter().collect())
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GenericAlgorithm::new(
            RouletteWheelSelection, 
            UniformCrossover, 
            GussianMutation::new(0.5, 2.0)
        );

        let mut population = vec![
            individual(vec![1.0, 1.0, 1.0, 1.0, 1.0]),
            individual(vec![1.0, 2.0, 2.0, 1.0, 1.0]),
            individual(vec![1.0, 3.0, 1.0, 1.0, 1.0]),
            individual(vec![2.0, 3.0, 2.0, 1.0, 1.0]),
            individual(vec![1.0, 1.0, 2.0, 3.0, 3.0]),
        ];

        for _ in 0..100 {
            population = ga.evolve(&mut rng, &population);
        }

        // 100 evolve with mutate
        let expected_population = vec![
            individual(vec![0.52645266, 2.4504585, 14.50473, 10.760423, 10.004925]), // 38.24699
            individual(vec![3.7930882, 3.641621, 16.658728, 9.183177, 10.004925]), // 43.28154
            individual(vec![0.60514677, 4.4005523, 17.480516, 10.760423, 9.906798]), // 43.15344
            individual(vec![4.1820974, 1.1582748, 14.313356, 6.3595815, 6.3362103]), // 32.349518
            individual(vec![3.8659909, 7.108799, 14.457064, 5.6994195, 7.4700184]), // 38.60129
        ];

        // 100 evolve without mutate
        // let expected_population = vec![
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        // ];

        // 10 evolve with mutate
        // let expected_population = vec![
        //     individual(vec![6.3359776, 2.0976896, 3.7707157, 0.7962358, 0.62884116]), // 13.62946
        //     individual(vec![3.1248477, 1.3918098, 4.009547, 0.7962358, -0.6071378]), // 8.715303
        //     individual(vec![6.939755, 2.9737287, 3.9434872, 1.7735945, -0.588717]), // 15.041849
        //     individual(vec![5.2996035, 1.1275165, 3.9434872, 1.5362605, 0.62884116]), // 12.535709
        //     individual(vec![4.9731855, 1.1275165, 5.2515707, 0.49221265, 0.62884116]), // 12.473327
        // ];

        // 10 evolve without mutate
        // let expected_population = vec![
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        //     individual(vec![1.0, 1.0, 2.0, 3.0, 3.0]), // 10.0
        //     individual(vec![1.0, 3.0, 2.0, 3.0, 3.0]), // 12.0
        // ];

        for individual in &expected_population {
            println!("fitness: {:?}", individual.chromosome().iter().sum::<f32>());
        }

        assert_eq!(population, expected_population);
            
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();
        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);
        let same_a = child.iter().zip(parent_a).filter(|(c, p)| *c == p).count();
        let same_b = child.iter().zip(parent_b.iter()).filter(|&(&c, &p)| c == p).count();
        assert_eq!(same_a, 51);
        assert_eq!(same_b, 49);
        println!("child:\n {:?}", child);
    }

    mod guassian_mutation {
        use super::*;

        fn actual(chance: f32, coeff:f32) -> Vec<f32>{
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            // let mut child = Chromosome::from_iter(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
            GussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
            child.into_iter().collect()
        }

        mod given_zero_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.0, coeff)
            }

            mod and_zero_coeff {
                use super::*;
                #[test]
                fn no_change_on_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeff {
                use super::*;
                #[test]
                fn no_change_on_original_chromosome() {
                    let actual = actual(2.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_half_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.5, coeff)
            }

            mod and_zero_coeff {
                use super::*;
                #[test]
                fn no_change_on_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeff {
                use super::*;
                #[test]
                fn slightly_change_on_original_chromosome() {
                    let actual = actual(2.0);
                    let expected = vec![1.0, 2.0, 3.8975005, 3.9868004, 5.8504343];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_max_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(1.0, coeff)
            }

            mod and_zero_coeff {
                use super::*;
                #[test]
                fn no_change_on_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeff {
                use super::*;
                #[test]
                fn entirely_change_on_original_chromosome() {
                    let actual = actual(2.0);
                    let expected = vec![-0.8181261, 1.5351684, 3.8975005, 4.19795, 6.4452357];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        
        let population = vec![
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let selected_fitness = RouletteWheelSelection
                                        .select(&mut rng, &population)
                                        .fitness() as i32;

            let stat = actual_histogram
                                .entry(selected_fitness)
                                .or_insert(0);
            *stat += 1;
        }

        let expected_histogram = BTreeMap::from_iter([
            (1, 102),
            (2, 198),
            (3, 301),
            (4, 399),
        ]);
        // instead of testing a exact number(might differ when population init order is different, we can check if freqency falls into a range)
        // assert!(900 <= *actual_histogram.get(&1).unwrap_or(&0) && *actual_histogram.get(&1).unwrap_or(&0) <= 1100);
        assert_eq!(actual_histogram, expected_histogram, "\nwe are testing comparison between \n{:?} \nand \n{:?}", actual_histogram, expected_histogram);
    }
}


