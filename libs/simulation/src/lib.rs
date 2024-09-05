
use nalgebra as na;
use neural_network::Network;


struct World {
    creatures: Vec<Creature>,
    environment: Environment,
}

struct Environment {
    foods: Vec<Food>,
}

struct Creature {
    body: Body,
    eye: Eye,
    brain: Brain,
    age: usize,
}

struct Body {
    size: f32,
    fat: f32,
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

struct Food {
    size: f32,
    position: na::Point2<f32>,
}

struct Eye {
    cell_num: usize,
    fov_range: f32,
    fov_angle: na::Rotation2<f32>,
}

// trait Sensor {
//     fn sense(environment: Environment) -> Vec<f32>;
// }


impl Eye {
    // fn see(environment: Environment, position: na::Point2<f32>, rotation: na::Rotation2<f32>) -> Vec<f32> {
    //     let vision_info = Vec::new();
    //     for food in environment.foods {
    //         if food in fov {
    //             cell_idx = 
    //         }
    //     }

    // }

    fn inside_fov<O>(&self, object_position: na::Point2<f32>, position: na::Point2<f32>, rotation: na::Rotation2<f32>) -> bool {
        let vector = position - object_position; // from creature to object
        let distance = vector.norm();
        if distance > self.fov_range {
            return false;
        }

        let object_angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vector).angle();
        let self_angle = rotation.angle();
        let fov_angle = self.fov_angle.angle();
        if object_angle < self_angle - fov_angle / 2.0 || object_angle > self_angle + fov_angle / 2.0 {
            return false;
        }

        true
    }
}

struct Brain {
    nn: Network
}

