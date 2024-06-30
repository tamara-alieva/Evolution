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
    position: Point2,
}

#[derive(Debug)]
pub struct Food { // Еда
    position: Point2,
}

#[derive(Debug)]
pub struct Point2 { // Точка
    x: f32,
    y: f32,
}