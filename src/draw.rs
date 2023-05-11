use crate::gene::Coefficient;
use crate::light::Light;
use crate::vehicle::{compose_pos, Vehicle, VEHICLE_RADIUS};
use macroquad::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

pub fn draw_scene(vehicles: &Vec<Vehicle>, lights: &Vec<Light>) {
    clear_background(SKYBLUE);
    for light in lights {
        draw_light(light);
    }
    for vehicle in vehicles {
        draw_vehicle(vehicle);
    }
}

pub fn draw_vehicle(vehicle: &Vehicle) {
    let mut color = Color::new(0.1, 0.1, 0.4, 1.0);
    let mut crossed = false;
    for gene in vehicle.genes() {
        if gene.sensor_and_engine_is_crossed() {
            // color.g = (color.g + 0.3).min(1.0);
            crossed = true;
        }
        match gene.coefficient {
            Coefficient::Excitatory => color.r = (color.r + 0.3).min(1.0),
            Coefficient::Inhibitory => color.g = (color.g + 0.3).min(1.0),
        }
    }
    let rotation = 45.0 + vehicle.angle();
    draw_poly(
        vehicle.position().x,
        vehicle.position().y,
        4,
        VEHICLE_RADIUS,
        rotation,
        color,
    );
    if crossed {
        let front_left = compose_pos(
            vehicle,
            Vec2::new(VEHICLE_RADIUS, VEHICLE_RADIUS) * FRAC_1_SQRT_2,
        );
        let front_right = compose_pos(
            vehicle,
            Vec2::new(VEHICLE_RADIUS, -VEHICLE_RADIUS) * FRAC_1_SQRT_2,
        );
        let back_left = compose_pos(
            vehicle,
            Vec2::new(-VEHICLE_RADIUS, VEHICLE_RADIUS) * FRAC_1_SQRT_2,
        );
        let back_right = compose_pos(
            vehicle,
            Vec2::new(-VEHICLE_RADIUS, -VEHICLE_RADIUS) * FRAC_1_SQRT_2,
        );
        draw_line(
            back_left.x,
            back_left.y,
            front_right.x,
            front_right.y,
            1.0,
            BLACK,
        );
        draw_line(
            back_right.x,
            back_right.y,
            front_left.x,
            front_left.y,
            1.0,
            BLACK,
        );
    }
}

pub fn draw_light(ligth: &Light) {
    // draw_circle_lines(ligth.position.x, ligth.position.y, ligth.radius, 2.0, DARKBROWN);
    // draw_circle_lines(ligth.position.x, ligth.position.y, ligth.radius, 2.0, YELLOW);

    let yellow_alpha = |alpha| Color::new(0.99, 0.98, 0.00, alpha);
    let draw_circle_radius_alpha = |radius, alpha| {
        draw_circle(
            ligth.position.x,
            ligth.position.y,
            ligth.radius * radius,
            yellow_alpha(alpha),
        );
    };
    draw_circle_radius_alpha(0.25, 0.05);
    draw_circle_radius_alpha(0.50, 0.05);
    draw_circle_radius_alpha(0.75, 0.05);
    draw_circle_radius_alpha(1.00, 0.2);
}
