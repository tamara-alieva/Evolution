use crate::*;

#[derive(Debug)]
pub struct Animal { // Животное
    pub(crate) position: na::Point2<f32>,      // позиция
    pub(crate) rotation: na::Rotation2<f32>,   // вращение
    pub(crate) speed: f32,                     // скорость
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}