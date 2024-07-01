use crate::*;

#[derive(Debug)]
pub struct Food { // Еда
    position: na::Point2<f32>,      // позиция
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}