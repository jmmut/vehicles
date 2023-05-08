use crate::gene::Gene;
use macroquad::prelude::Vec2;

pub struct Vehicle {
    genes: Vec<Gene>,
    left_motor_activation: f32,
    right_motor_activation: f32,
    pub position: Vec2,
    pub angle: f32,
}

impl Vehicle {
    pub fn new(genes: Vec<Gene>, position: Vec2, angle: f32) -> Self {
        Self {
            genes,
            left_motor_activation: 0.0,
            right_motor_activation: 0.0,
            position,
            angle,
        }
    }
}
