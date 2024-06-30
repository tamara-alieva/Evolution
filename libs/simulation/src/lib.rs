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
    pub fn random(rng: &mut dyn RngCore) -> Self { // конструктор-рандомайзер
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World { // геттер (мир)
        &self.world
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self { // конструктор-рандомайзер
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self { // конструктор-рандомайзер
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

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self { // конструктор-рандомайзер
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}