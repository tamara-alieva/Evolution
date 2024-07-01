use crate::*;
use std::{f32::consts::*, sync::PoisonError};

const FOV_RANGE: f32 = 0.25; // видимость пищи (расстояние от птицы до пищи)
const FOV_ANGLE: f32 = PI + FRAC_PI_4; // угол обзора (5/4 * pi)
const CELLS: usize = 9; // количество фоторецепторов

#[derive(Debug)]
pub struct Eye { // Глаз
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range:f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist > self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}