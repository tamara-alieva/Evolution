use crate::*;

#[derive(Debug)]
pub struct Animal { // Животное
    pub(crate) position: na::Point2<f32>,      // позиция
    pub(crate) rotation: na::Rotation2<f32>,   // вращение
    pub(crate) speed: f32,                     // скорость
    pub(crate) eye: Eye,                       // глаз
    pub(crate) brain: nn::Network,             // мозг (нейронная сеть)
    pub(crate) satiation: usize,               // количество съеденной пищи
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
                nn::LayerTopology { // слой ввода
                    neurons: eye.cells(),
                },

                nn::LayerTopology { // скрытый слой
                    neurons: 2 * eye.cells(),
                },

                nn::LayerTopology { neurons: 2 }, // слой вывода
            ]
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}