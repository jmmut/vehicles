use crate::gene::{Coefficient, Gene, Side};
use crate::light::Light;
use macroquad::prelude::Vec2;
use std::f32::consts::{FRAC_1_PI, PI};

// const SENSOR_MAX_DISTANCE: f32 = 50.0;

pub const VEHICLE_RADIUS: f32 = 15.0;

pub struct Vehicle {
    pub genes: Vec<Gene>,
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
    const INTENSITY_TO_STIMULUS: f32 = 0.01;
    for light in lights {
        for gene in &vehicle.genes {
            let relative_sensor_pos = match gene.sensor_side {
                Side::Left => Vec2::new(0.0, VEHICLE_RADIUS),
                Side::Right => Vec2::new(0.0, -VEHICLE_RADIUS),
            };
            let sensor_pos = compose_pos(vehicle, relative_sensor_pos);
            let intensity = sensor_intensity(sensor_pos, light, &gene.coefficient);
            let stimulus = intensity * INTENSITY_TO_STIMULUS;
            match gene.motor_connection {
                Side::Left => vehicle.left_motor_activation += stimulus,
                Side::Right => vehicle.right_motor_activation += stimulus,
            }
        }
    }
}

pub fn compose_pos(vehicle: &Vehicle, relative_pos: Vec2) -> Vec2 {
    vehicle.position + relative_pos.rotate(angle_to_cartesian(vehicle.angle))
}

fn sensor_intensity(sensor_position: Vec2, light: &Light, coef: &Coefficient) -> f32 {
    let sensor_to_light = light.position - sensor_position;
    let intensity = match coef {
        Coefficient::Excitatory => light.radius - sensor_to_light.length(),
        Coefficient::Inhibitory => sensor_to_light.length(),
    };
    intensity.max(0.0)
}

pub fn advance_vehicle(vehicle: &mut Vehicle) {
    let curve = vehicle.left_motor_activation - vehicle.right_motor_activation;
    let distance = vehicle.left_motor_activation + vehicle.right_motor_activation + 0.5;
    let new_half_angle = vehicle.angle + curve;
    let new_angle = vehicle.angle + curve * 2.0;
    let new_pos = vehicle.position + distance * angle_to_cartesian(new_half_angle);
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
    use super::*;
    use crate::assertions::{assert_float_eq, assert_vec2_eq};
    use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

    #[test]
    fn test_angles() {
        assert_vec2_eq(angle_to_cartesian(0.0), Vec2::new(1.0, 0.0));
        assert_vec2_eq(
            angle_to_cartesian(45.0),
            Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(90.0), Vec2::new(0.0, 1.0));
        assert_vec2_eq(
            angle_to_cartesian(90.0 + 45.0),
            Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(180.0), Vec2::new(-1.0, 0.0));
        assert_vec2_eq(
            angle_to_cartesian(180.0 + 45.0),
            Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(270.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(angle_to_cartesian(-90.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(
            angle_to_cartesian(270.0 + 45.0),
            Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
        assert_vec2_eq(
            angle_to_cartesian(-45.0),
            Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
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

    const INITIAL_ANGLE: f32 = 45.0;

    #[test]
    fn test_advance_no_stimuli() {
        let mut vehicle = empty_vehicle();
        advance_vehicle(&mut vehicle);
        assert_eq!(vehicle.angle, INITIAL_ANGLE);
        assert_eq!(vehicle.position, Vec2::default());
    }

    fn empty_vehicle() -> Vehicle {
        Vehicle {
            genes: vec![],
            left_motor_activation: 0.0,
            right_motor_activation: 0.0,
            position: Vec2::default(),
            angle: INITIAL_ANGLE,
        }
    }
}
