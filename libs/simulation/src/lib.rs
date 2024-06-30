use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation { // Симуляция
    world: World,
}

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl World {
    pub(crate) fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }
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

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
        }
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            // ------ ^-------^
            // | If not for `rand-no-std`, we'd have to do awkward
            // | `na::Point2::new(rng.gen(), rng.gen())` instead
            // ---

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