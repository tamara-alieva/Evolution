use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation { // Симуляция
    world: World,
}

#[derive(Debug)]
pub struct World { // Мир
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal { // Животное
    position: na::Point2<f32>,      // позиция
    rotation: na::Rotation2<f32>,   // вращение
    speed: f32,                     // скорость
}

#[derive(Debug)]
pub struct Food { // Еда
    position: na::Point2<f32>,      // позиция
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
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| food::random(rng))
            .collect();

        Self { animals, foods }
    }
}

