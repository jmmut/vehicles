use std::f32::consts::{FRAC_1_PI, PI};
use crate::gene::Gene;
use macroquad::prelude::Vec2;
use crate::light::Light;

// const SENSOR_MAX_DISTANCE: f32 = 50.0;

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

pub fn stimulate(vehicle: &mut Vehicle, lights: &[Light]) {
    for light in lights {
        let vehicle_to_light = light.position - vehicle.position;
        if vehicle_to_light.length() < light.radius {
            let ligth_angle = cartesian_to_angle_degrees(vehicle_to_light);
            if ligth_angle.abs() <= 90.0 {
                let stimulus = (light.radius - vehicle_to_light.length()) * 0.01;
                // TODO: this ignore genes
                if ligth_angle >= 0.0 {
                    vehicle.left_motor_activation += stimulus;
                } else {
                    vehicle.right_motor_activation += stimulus;
                }
            }
        }
    }
}

pub fn advance_vehicle(vehicle: &mut Vehicle) {
    let curve = vehicle.left_motor_activation - vehicle.right_motor_activation;
    let distance = vehicle.left_motor_activation + vehicle.right_motor_activation;
    let new_angle = curve * 2.0;
    let new_pos = vehicle.position + distance * angle_to_cartesian(curve);
    vehicle.angle = new_angle;
    vehicle.position = new_pos;
    vehicle.left_motor_activation = 0.0;
    vehicle.right_motor_activation = 0.0;
}

fn angle_to_cartesian(angle_degrees: f32) -> Vec2 {
    let angle_radians = 2.0 * PI * angle_degrees * (1.0 / 360.0);
    Vec2::new(angle_radians.cos(), angle_radians.sin())
}

fn cartesian_to_angle_degrees(vector: Vec2) -> f32 {
    f32::atan2(vector.y, vector.x) * 360.0 * 0.5 * FRAC_1_PI
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};
    use crate::assertions::{assert_float_eq, assert_vec2_eq};
    use super::*;

    #[test]
    fn test_angles() {
        assert_vec2_eq(angle_to_cartesian(0.0), Vec2::new(1.0, 0.0));
        assert_vec2_eq(angle_to_cartesian(45.0), Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2));
        assert_vec2_eq(angle_to_cartesian(90.0), Vec2::new(0.0, 1.0));
        assert_vec2_eq(angle_to_cartesian(90.0 + 45.0), Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2));
        assert_vec2_eq(angle_to_cartesian(180.0), Vec2::new(-1.0, 0.0));
        assert_vec2_eq(angle_to_cartesian(180.0 + 45.0), Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_vec2_eq(angle_to_cartesian(270.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(angle_to_cartesian(-90.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(angle_to_cartesian(270.0 + 45.0), Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert_vec2_eq(angle_to_cartesian(-45.0), Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    fn assert_angle_roundtrip(expected_angle: f32) {
        let cartesian = angle_to_cartesian(expected_angle);
        let angle = cartesian_to_angle_degrees(cartesian);
        assert_float_eq(angle, expected_angle);
    }

    #[test]
    fn test_angles_roundtrip() {
        assert_angle_roundtrip(0.0);
        assert_angle_roundtrip(40.0);
        assert_angle_roundtrip(45.0);
        assert_angle_roundtrip(50.0);
        assert_angle_roundtrip(85.0);
        assert_angle_roundtrip(90.0);
        assert_angle_roundtrip(95.0);
        assert_angle_roundtrip(90.0 + 40.0);
        assert_angle_roundtrip(90.0 + 45.0);
        assert_angle_roundtrip(90.0 + 50.0);
        assert_angle_roundtrip(175.0);
        assert_angle_roundtrip(179.0);
        assert_angle_roundtrip(-175.0);
        assert_angle_roundtrip(180.0 + 40.0 - 360.0);
        assert_angle_roundtrip(180.0 + 45.0 - 360.0);
        assert_angle_roundtrip(180.0 + 50.0 - 360.0);
        assert_angle_roundtrip(-90.0);
        assert_angle_roundtrip(-50.0);
        assert_angle_roundtrip(-45.0);
        assert_angle_roundtrip(-40.0);
    }
}
