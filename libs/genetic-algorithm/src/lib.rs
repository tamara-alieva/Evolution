use rand::seq::{IteratorRandom, SliceRandom};
use rand::{Rng, RngCore};
use std::ops::Index;
use std::iter::FromIterator;

pub struct GeneticAlgorithm<S> { // Генетический алгоритм
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
}

pub trait Individual { // Индивид
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

impl<S> GeneticAlgorithm<S> 
where 
    S: SelectionMethod,
{   
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
    ) -> Self {
        Self { 
            selection_method,
            crossover_method: Box::new(crossover_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> 
    where
        I: Individual 
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // мутация
                todo!()
            })
            .collect()
    }
}


pub trait SelectionMethod { // Метод селекции
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where 
    I: Individual;
}

pub struct RouletteWheelSelection; // Колесо-рулетка

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where 
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome { // Хромосома
    genes: Vec<f32>
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

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub trait CrossoverMethod { // Метод для скрещивания
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct GaussianMutation { // Мутация Гаусса
    chance: f32, // вероятность изменения набора генов: 0.0 -> нет изменений, 1.0 -> изменены все гены
    coeff: f32, // магнитуда изменения: 0.0 -> нет изменений, 
                // 3.0 -> выбранные гены будут увеличены или уменьшены, как минимум, на 3.0
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0); // проверка на диапазон значений вероятности

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn roulette_wheel_selection() { // выбор по колесу-рулетке
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram
                .entry(fitness)
                .or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter([
            (1, 98), // (здоровье, сколько раз оно уже было выбрано)
            (2, 202),
            (3, 278),
            (4, 422),
        ]);

        assert_eq!(actual_histogram, expected_histogram);

    }

    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &Chromosome {
            panic!("not supported for TestIndividual")
        }
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        // Число генов потомка, отличающихся от генов родителя
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 49); // Это число должно отл
        assert_eq!(diff_b, 51);
    }

    mod gaussian_mutation { // тест алгоритма мутации Гаусса

        use std::{hash::DefaultHasher, process::Child};

        use super::*;
        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }

        mod given_zero_chance {

            mod and_zero_coefficient {
                #[test]
                fn does_not_change_the_original_chromosome() {
                    todo!();
                }
            }

            mod and_nonzero_coefficient {
                #[test]
                fn does_not_change_the_original_chromosome() {
                    todo!();
                }
            }

        }

        mod given_fifty_fifty_chance {

            mod and_zero_coefficient {
                #[test]
                fn does_not_change_the_original_chromosome() {
                    todo!();
                }
            }

            mod and_nonzero_coefficient {
                #[test]
                fn slightly_changes_the_original_chromosome() {
                    todo!();
                }
            }
        }

        mod given_max_chance {
            mod and_zero_coefficient {
                #[test]
                fn does_not_change_the_original_chromosome() {
                    todo!();
                }
            }
            mod and_nonzero_coefficient {
                #[test]
                fn entirely_changes_the_original_chromosome() {
                    todo!();
                }
            }
        }
    }

}