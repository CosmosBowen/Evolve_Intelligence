use std::f32::consts::FRAC_PI_4;
use std::f32::consts::PI;

use nalgebra as na;
use ggez::graphics::Color;
use rand::RngCore;

use neural_network::{Network, LayerTopology};
use crate::eye::*;
use crate::food::Food;

pub const SPEED_MIN: f32 = 1.0;
pub const SPEED_MAX: f32 = 10.0;

const SPEED_ACCEL: f32 = 2.0;
const ROTATION_ACCEL: f32 = PI * 2.0 / 3.0 ;

pub const CREATURE_SIZE: f32 = 20.0;
pub const FOOD_SIZE: f32 = CREATURE_SIZE / 3.0;

pub const NECK_SIZE_RATIO: f32 = 2.0;

pub const EYE_SIZE: f32 = CREATURE_SIZE / 3.0;
pub const EYEBALL_SIZE: f32 = CREATURE_SIZE / 5.0;
const EYE_DISTANCE_RATIO: f32 = 0.5;
const EYEBALL_DISTANCE_RATIO: f32 = EYE_DISTANCE_RATIO + 0.05;

pub const MOUTH_SIZE: f32 = CREATURE_SIZE / 5.0;
pub const MOUTH_OPEN_ANGLE: f32 = 10.0 * PI / 180.0; // 10 degree
const MOUTH_DISTANCE_RATIO: f32 = 0.95;

pub const BODY_LENGTH: f32 = CREATURE_SIZE * NECK_SIZE_RATIO; 
pub const EYE_POSITION : f32 = CREATURE_SIZE * NECK_SIZE_RATIO * EYE_DISTANCE_RATIO;
pub const EYEBALL_POSITION : f32 = CREATURE_SIZE * NECK_SIZE_RATIO * EYEBALL_DISTANCE_RATIO;
pub const MOUTH_POSITION : f32 = CREATURE_SIZE * NECK_SIZE_RATIO * MOUTH_DISTANCE_RATIO;

const FOOD_EATEN_DISTANCE: f32 = CREATURE_SIZE / 1.5;
const MAX_EAT: i8 = 50;

pub const CREATURE_EYE_CELLS: usize = 9;
pub const CREATURE_EYE_ANGLE: f32 = PI + FRAC_PI_4;
pub const CREATURE_EYE_RANGE: f32 = 1000.0; // CREATURE_SIZE * 25.0;

pub struct Creature {
    pub position: na::Point2<f32>,
    pub rotation: f32, // radians // clockwise, start from south
    pub speed: f32,
    pub eat: u32,
    pub color: Color,
    pub eye: Eye,
    pub brain: Network,
}

impl Creature {
    pub fn new(position: na::Point2<f32>, rotation: f32, speed: f32, rng: &mut dyn RngCore, optional_brain: Option<Network>) -> Self {
        let eye = Eye::new(CREATURE_EYE_RANGE, CREATURE_EYE_ANGLE, CREATURE_EYE_CELLS);
        let cells = eye.cells();
        let brain = if let Some(brain) = optional_brain {
            brain
        } else {
            Network::random(rng, &[ // 5 3 
                LayerTopology {num_neuron: cells}, // how to use eye.cells() here?
                LayerTopology {num_neuron: 5},
                LayerTopology {num_neuron: 3},
                LayerTopology {num_neuron: 2},
            ])
        };

        Self { 
            position, 
            rotation, 
            speed, 
            eat: 0, 
            color: Color::WHITE,
            eye,
            brain,
        }
    }

    pub fn move_for_foods(&mut self, foods: &Vec<Food>) {
        let vision_info = self.see(foods);
        let actions = self.decide(vision_info);
        self.move_body(actions);
    }

    fn see(&self, foods: &Vec<Food>) -> Vec<f32> {
        self.eye.process_vision(self.position, self.rotation, foods)
    }

    fn decide(&self, vision_info: Vec<f32>) -> Vec<f32> {
        self.brain.propagate(vision_info)
    }

    fn move_body(&mut self, actions: Vec<f32>) {
        let rotation_chage = actions[0].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
        let speed_change = actions[1].clamp(-SPEED_ACCEL, SPEED_ACCEL);

        // self.rotation += rotation_chage;
        self.rotation += rotation_chage;
        // self.rotation = na::wrap(self.rotation + rotation_chage, -PI, PI);
        self.speed = (self.speed + speed_change).clamp(SPEED_MIN, SPEED_MAX);

        let dx = self.speed * self.rotation.sin();
        let dy = - self.speed * self.rotation.cos();

        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn eat(&mut self, food: &Food) -> bool {
        let mouth_position = nalgebra::Point2::new(
            self.position.x + self.rotation.sin() * MOUTH_POSITION, 
            self.position.y - self.rotation.cos() * MOUTH_POSITION);
        let distance = na::distance(&mouth_position, &food.position);
        if distance <= FOOD_EATEN_DISTANCE {
            self.eat += 1;
            let color_intensity = (self.eat as f32 / (MAX_EAT as f32)).min(1.0);
            self.color = Color::new(1.0, 1.0, 1.0 - color_intensity, 1.0);
            true
        } else {
            false
        }
    }
}