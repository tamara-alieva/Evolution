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

//-----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    const TEST_EYE_CELLS: usize = 13;

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle,TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            let actual_vision: Vec<_> = actual_vision
            .into_iter()
            .map(|cell| {
                if cell >= 0.7 {
                    // <0.7, 1.0> - пища прямо перед птицей
                    "#"
                } else if cell >= 0.3 {
                    // <0.3, 0.7) - пища чуть дальше
                    "+"
                } else if cell > 0.0 {
                    // <0.0, 0.3) - пища довольно далеко
                    "."
                } else {
                    // 0.0 - фоторецептор "не видит" пищу
                    " "
                }
            })
            .collect();

            let actual_vision = actual_vision.join("");
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    // тестирования удаления пищи от птицы на расстояние
    #[test_case(1.0, "      +      ")] // пища в поле зрения
    #[test_case(0.9, "      +      ")]
    #[test_case(0.8, "      +      ")]
    #[test_case(0.7, "      .      ")] 
    #[test_case(0.6, "      .      ")]
    #[test_case(0.5, "             ")] // пища исчезает из поля зрения
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]    
    fn fov_ranges(fov_range: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            fov_range,
            expected_vision,
        }.run()
    }

    // тестирование вращения
    #[test_case(0.00 * PI, "         +   ")] // пища правее
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")] // пища перед птицей
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")] // пища левее
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")] // пища позади птицы (птица видит её, т.к. fov_angle = 360 гр.)
    #[test_case(1.75 * PI, "           + ")]
    #[test_case(2.00 * PI, "         +   ")]
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn rotations(rot: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rot,
            expected_vision,
        }.run()
    }

    // тестирование позиции
    // по оси X
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    // по оси Y
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn positions(x: f32, y: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rot: 3.0 * FRAC_PI_2,
            x,
            y,
            expected_vision,
        }.run()
    }

    // тестирование угла обзора
    #[test_case(0.25 * PI, " +         + ")]
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")]
    #[test_case(1.00 * PI, "   . + + .   ")]
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")]
    fn fov_angles(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            fov_range: 1.0,
            x: 0.5,
            y: 0.5,
            rot: 3.0 * FRAC_PI_2,
            fov_angle,
            expected_vision,
        }.run()
    }

}