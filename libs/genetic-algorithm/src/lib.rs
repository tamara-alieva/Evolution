pub struct GeneticAlgorithm; // Генетический алгоритм

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // селекция
                // скрещивание
                // мутация
                todo!()
            })
            .collect()
    }
}