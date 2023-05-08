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
