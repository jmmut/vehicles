use crate::gene::{Coefficient, Gene, Genes, Side};
use crate::light::Light;
use macroquad::prelude::Vec2;

use crate::math::angle_to_cartesian;

// const SENSOR_MAX_DISTANCE: f32 = 50.0;

pub const VEHICLE_RADIUS: f32 = 15.0;
const MINIMUM_SPEED: f32 = 0.5;

#[derive(Debug)]
pub struct Vehicle {
    genes: Genes,
    left_engine_activation: f32,
    right_engine_activation: f32,
    pub position: Vec2,
    pub angle: f32,
}

impl Vehicle {
    pub fn new(genes: Vec<Gene>, position: Vec2, angle: f32) -> Self {
        Self {
            genes,
            left_engine_activation: 1.0,
            right_engine_activation: 1.0,
            position,
            angle,
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut lines = vec![];
        for gene in &self.genes {
            lines.push(format!("{:?}", gene));
        }
        lines.push(format!(
            "left engine: {}, right engine {}, pos: {}, angle: {}",
            self.left_engine_activation, self.right_engine_activation, self.position, self.angle
        ));
        lines
    }

    pub fn genes(&self) -> &Genes {
        &self.genes
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
            match gene.engine_connection {
                Side::Left => vehicle.left_engine_activation += stimulus,
                Side::Right => vehicle.right_engine_activation += stimulus,
            }
        }
    }
}

pub fn compose_pos(vehicle: &Vehicle, relative_pos: Vec2) -> Vec2 {
    vehicle.position + relative_pos.rotate(angle_to_cartesian(vehicle.angle))
}

fn sensor_intensity(sensor_position: Vec2, light: &Light, coef: &Coefficient) -> f32 {
    let sensor_to_light = light.position - sensor_position;
    // TODO: limit light distance?
    let intensity = match coef {
        Coefficient::Excitatory => light.radius - sensor_to_light.length(),
        Coefficient::Inhibitory => sensor_to_light.length(),
    };
    intensity.max(0.0)
}

pub fn advance_vehicle(vehicle: &mut Vehicle) {
    let curve = vehicle.right_engine_activation - vehicle.left_engine_activation;
    let distance = vehicle.right_engine_activation + vehicle.left_engine_activation + MINIMUM_SPEED;
    let new_half_angle = vehicle.angle + curve;
    let new_angle = vehicle.angle + curve * 2.0;
    let new_pos = vehicle.position + distance * angle_to_cartesian(new_half_angle);
    vehicle.angle = new_angle;
    vehicle.position = new_pos;
    vehicle.left_engine_activation = 0.0;
    vehicle.right_engine_activation = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    const INITIAL_ANGLE: f32 = 45.0;

    #[test]
    fn test_advance_no_stimuli() {
        let mut vehicle = empty_vehicle();
        advance_vehicle(&mut vehicle);
        assert_eq!(vehicle.angle, INITIAL_ANGLE);
        assert_eq!(vehicle.position, MINIMUM_SPEED * angle_to_cartesian(INITIAL_ANGLE));
    }

    fn empty_vehicle() -> Vehicle {
        Vehicle {
            genes: vec![],
            left_engine_activation: 0.0,
            right_engine_activation: 0.0,
            position: Vec2::default(),
            angle: INITIAL_ANGLE,
        }
    }

    #[test]
    fn test_advance_stimuli() {
        let mut vehicle = empty_vehicle();
        vehicle.left_engine_activation += 2.0;
        vehicle.right_engine_activation += 1.0;
        advance_vehicle(&mut vehicle);
        assert_eq!(vehicle.angle, INITIAL_ANGLE - 2.0);
    }
}
