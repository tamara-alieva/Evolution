use crate::*;

#[derive(Debug)]
pub struct Animal { // Животное
    pub(crate) position: na::Point2<f32>,      // позиция
    pub(crate) rotation: na::Rotation2<f32>,   // вращение
    pub(crate) speed: f32,                     // скорость
    pub(crate) eye: Eye,                       // глаз
    pub(crate) brain: Brain,                   // мозг (нейронная сеть)
    pub(crate) satiation: usize,               // количество съеденной пищи
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
    ) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
}