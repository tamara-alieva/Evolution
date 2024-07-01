mod animal;
mod food;
mod world;

pub use self::{animal::*, food::*, world::*};

use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation { // Симуляция
    world: World,
}

#[derive(Debug)]
pub struct Point2 { // Точка
    x: f32,
    y: f32,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}