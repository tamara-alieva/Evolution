use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub ga: ga::Statistics,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Поколение {}:", self.generation)?;
        write!(
            f,
            "мин[{:.2}] макс[{:.2}] сред.[{:.2}] медиан.[{:.2}]",
            self.ga.min_fitness(),
            self.ga.max_fitness(),
            self.ga.avg_fitness(),
            self.ga.median_fitness()
        )
    }
}
