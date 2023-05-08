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
            left_motor_activation: 1.0,
            right_motor_activation: 1.0,
            position,
            angle,
        }
    }
}

pub fn advance_vehicle(vehicle: &mut Vehicle) {
    let curve = vehicle.left_motor_activation - vehicle.right_motor_activation;
    let distance = vehicle.left_motor_activation + vehicle.right_motor_activation;
    let new_angle = curve * 2.0;
    let new_pos = vehicle.position + distance * Vec2::new(curve.cos(), curve.sin());
    vehicle.angle = new_angle;
    vehicle.position = new_pos;
}
