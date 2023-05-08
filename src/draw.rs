use crate::light::Light;
use crate::vehicle::{Vehicle, VEHICLE_RADIUS};
use macroquad::prelude::*;

pub fn draw_vehicle(vehicle: &Vehicle) {
    let color = DARKBLUE;
    let rotation = 45.0 + vehicle.angle;
    draw_poly(
        vehicle.position.x,
        vehicle.position.y,
        4,
        VEHICLE_RADIUS,
        rotation,
        color,
    );
}

pub fn draw_light(ligth: &Light) {
    // draw_circle_lines(ligth.position.x, ligth.position.y, ligth.radius, 2.0, DARKBROWN);
    // draw_circle_lines(ligth.position.x, ligth.position.y, ligth.radius, 2.0, YELLOW);
    draw_circle(
        ligth.position.x,
        ligth.position.y,
        ligth.radius * 0.25,
        Color::new(0.99, 0.98, 0.00, 0.05),
    );
    draw_circle(
        ligth.position.x,
        ligth.position.y,
        ligth.radius * 0.5,
        Color::new(0.99, 0.98, 0.00, 0.05),
    );
    draw_circle(
        ligth.position.x,
        ligth.position.y,
        ligth.radius * 0.75,
        Color::new(0.99, 0.98, 0.00, 0.05),
    );
    draw_circle(
        ligth.position.x,
        ligth.position.y,
        ligth.radius,
        Color::new(0.99, 0.98, 0.00, 0.2),
    )
}
