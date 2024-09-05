use std::f32::consts::PI;

use crate::*;
use nalgebra as na;
// use std::f32::consts::PI;

const FOV_RANGE: f32 = 20.0;
const FOV_ANGLE: f32 = 90.0;
const CELLS:usize = 5; // 13

pub struct Eye {
    fov_range: f32,
    fov_angle: f32, // radians
    cells: usize,
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {fov_range, fov_angle, cells}
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self, 
        position: na::Point2<f32>, 
        rotation: f32, // radians
        foods: &Vec<Food>
    ) -> Vec<f32> {
        let mut vision_info = vec![0.0; self.cells];
        let fov_angle = self.fov_angle;
        for food in foods {
            // within range
            let vec = food.position - position;
            let dist = vec.norm();
            if dist > self.fov_range {
                continue;
            }

            // within angle
            let mut food_angle = na::Rotation2::rotation_between(
                &na::Vector2::y(),
                &vec, 
            ).angle(); // axis upwards, up up, not as render y axis points down
            food_angle -= rotation;
            food_angle = na::wrap(food_angle, -PI, PI);
            
            if food_angle < -fov_angle / 2.0 || food_angle > fov_angle / 2.0 {
                continue;
            }

            // which cell
            let cell_idx = food_angle - (-fov_angle / 2.0);
            let cell_idx = cell_idx / fov_angle;

            let cell_idx = cell_idx * (self.cells as f32);

            let cell_idx = (cell_idx as usize).min(self.cells - 1);
            
            // // instead, just switch it on if it's off
            // if vision_info[cell_idx] == 0.0 {
            //     vision_info[cell_idx] = 1.0;
            // }

            // add value from dist
            let energy = (self.fov_range - dist) / self.fov_range;
            vision_info[cell_idx] += energy;
        }

        vision_info

    }
}



const EYE_CELLS: usize = 13; //13

fn food(x: f32, y: f32) -> Food {
    Food {
        position: na::Point2::new(x, y),
        is_eaten: false,
    }
}


// Run test in terminal
// cargo test -p evolution_sim
#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;
    use test_case::test_case;

    struct TestCase {
        fov_range: f32,
        fov_angle: f32,
        foods: Vec<Food>,
        x: f32,
        y: f32,
        rotation: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn make_human_readable(&self, vision_info: Vec<f32>) -> String{
            print!("vision_info: {:?}", vision_info);
            vision_info
            .into_iter()
            .map(|cell| {
                if cell >= 0.7 {
                    "#"
                } else if cell >= 0.3 {
                    "+"
                } else if cell > 0.0 {
                    "."
                } else {
                    " "
                }
            })
            .collect::<Vec<&str>>()
            .join("")
        }
        
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, EYE_CELLS);
            let actual_vision_info = eye.process_vision(na::Point2::new(self.x, self.y), self.rotation, &self.foods);
            let actual_vision = self.make_human_readable(actual_vision_info);
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    // #[test_case(10.0,   "      +      ")]
    // #[test_case(9.0,    "      +      ")]
    // #[test_case(8.0,    "      +      ")]
    // #[test_case(7.0,    "      .      ")]
    // #[test_case(6.0,    "      .      ")]
    // #[test_case(5.0,    "             ")]
    // #[test_case(4.0,    "             ")]
    // #[test_case(3.0,    "             ")]
    // #[test_case(2.0,    "             ")]
    // #[test_case(1.0,    "             ")]
    fn fov_ranges(fov_range: f32, expected_vision: &'static str) {
        TestCase{
            fov_range,
            fov_angle: 90.0,
            foods: vec![food(5.0, 10.0)],
            x: 5.0,
            y: 5.0,
            rotation: 0.0,
            expected_vision,
        }.run();
    }

    // #[test_case(0.0,    "         +   ")]
    // #[test_case(30.0,   "        +    ")]
    // #[test_case(60.0,   "       +     ")]
    // #[test_case(90.0,   "      +      ")]
    // #[test_case(120.0,  "     +       ")]
    // #[test_case(150.0,  "    +        ")]
    // #[test_case(180.0,  "   +         ")]
    // #[test_case(210.0,  "  +          ")]
    // #[test_case(240.0,  " +           ")]
    // #[test_case(270.0,  "+            ")]
    // #[test_case(290.0,  "            +")]
    // #[test_case(300.0,  "           + ")]
    // #[test_case(330.0,  "          +  ")]
    // #[test_case(360.0,  "         +   ")]
    // #[test_case(390.0,  "        +    ")]
    fn fov_angle(rotation: f32, expected_vision: &'static str) {
        TestCase {
            fov_range: 10.0,
            fov_angle: 360.0,
            foods: vec![food(0.0, 5.0)],
            x: 5.0,
            y: 5.0,
            rotation,
            expected_vision,
        }.run();
    }

    // #[test_case(0.0,    "            .")]
    // #[test_case(1.0,    "            +")]
    // #[test_case(2.0,    "          +  ")]
    // #[test_case(3.0,    "         +   ")]
    // #[test_case(4.0,    "        +    ")]
    // #[test_case(5.0,    "      +      ")]
    // #[test_case(6.0,    "    +        ")]
    // #[test_case(7.0,    "   +         ")]
    // #[test_case(8.0,    "  +          ")]
    // #[test_case(9.0,    "+            ")]
    // #[test_case(10.0,   ".            ")]
    fn y(y: f32, expected_vision: &'static str) {
        TestCase {
            fov_range: 10.0,
            fov_angle: 90.0,
            foods: vec![food(10.0, 5.0)],
            x: 5.0,
            y,
            rotation: - PI / 2.0, // 3.0 / 2.0 * PI
            expected_vision,
        }.run();
    }

    // #[test_case(0.0,    "            .")]
    // #[test_case(1.0,    "            +")]
    // #[test_case(2.0,    "          +  ")]
    // #[test_case(3.0,    "         +   ")]
    // #[test_case(4.0,    "        +    ")]
    // #[test_case(5.0,    "      +      ")]
    // #[test_case(6.0,    "    +        ")]
    // #[test_case(7.0,    "   +         ")]
    // #[test_case(8.0,    "  +          ")]
    // #[test_case(9.0,    "+            ")]
    // #[test_case(10.0,   "             ")]
    fn x(x: f32, expected_vision: &'static str) {
        TestCase {
            fov_range: 10.0,
            fov_angle: 90.0,
            foods: vec![food(5.0, 0.0)],
            x,
            y: 5.0,
            rotation: PI, // - PI
            expected_vision,
        }.run();
    }

    // #[test_case(30.0,   "             ")]
    // #[test_case(60.0,   "             ")]
    // #[test_case(90.0,   ".            ")]
    // #[test_case(120.0,  " .           ")]
    // #[test_case(150.0,  "  .          ")]
    // #[test_case(180.0,  "   .         ")]
    // #[test_case(210.0,  "   .         ")]
    // #[test_case(240.0,  "    .        ")]
    // #[test_case(270.0,  "    .        ")]
    // #[test_case(290.0,  "    .        ")]
    // #[test_case(300.0,  "    .        ")]
    // #[test_case(330.0,  "    .        ")]
    // #[test_case(360.0,  "    .        ")]
    fn fov_angles(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            fov_range: 10.0,
            fov_angle,
            foods: vec![food(10.0, 0.0)],
            x: 5.0,
            y: 5.0,
            rotation: - PI/2.0,
            expected_vision,
        }.run();
    }

    // // move horizontal from right to lefts
    // #[test_case(9.0, 5.0, "#            ")]
    // #[test_case(8.0, 5.0, "  #       #  ")]
    // #[test_case(7.0, 5.0, "   +     +   ")]
    // #[test_case(6.0, 5.0, "    +   +    ")]
    // #[test_case(5.0, 5.0, "    +   +    ")]
    // #[test_case(4.0, 5.0, "     + +     ")]
    // #[test_case(3.0, 5.0, "     . .     ")]
    // #[test_case(2.0, 5.0, "     . .     ")]
    // #[test_case(1.0, 5.0, "     . .     ")]
    // #[test_case(0.0, 5.0, "             ")]

    // // move vertical from top to downs
    // #[test_case(5.0, 0.0, "            +")]
    // #[test_case(5.0, 1.0, "          +  ")]
    // #[test_case(5.0, 2.0, "         +  +")]
    // #[test_case(5.0, 3.0, "        + +  ")]
    // #[test_case(5.0, 4.0, "      +  +   ")]
    // #[test_case(5.0, 6.0, "   +  +      ")]
    // #[test_case(5.0, 7.0, "  + +        ")]
    // #[test_case(5.0, 8.0, "+  +         ")]
    // #[test_case(5.0, 9.0, ". +          ")]
    // #[test_case(5.0, 10.0, "+            ")]
    fn positions(x: f32, y: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(10.0, 4.0), food(10.0, 6.0)],
            fov_range: 10.0,
            fov_angle: 90.0,
            rotation: 3.0 * FRAC_PI_2,
            x,
            y,
            expected_vision,
        }.run()
    }


    #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
    #[test_case(0.50 * PI, ".  +     +   ")]
    #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
    #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.    ")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is the widest = 8 foods
    fn fov_angles_test(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 3.33),
                food(0.0, 6.66),
                food(0.0, 10.0),
                food(10.0, 0.0),
                food(10.0, 3.33),
                food(10.0, 6.66),
                food(10.0, 10.0),
            ],
            fov_range: 10.0,
            fov_angle,
            x: 5.0,
            y: 5.0,
            rotation: 3.0 * FRAC_PI_2,
            expected_vision,
        }.run()
    }


}