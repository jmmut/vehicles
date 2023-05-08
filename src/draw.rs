use crate::vehicle::Vehicle;
use macroquad::prelude::*;

const VEHICLE_RADIUS: f32 = 15.0;

pub fn draw_vehicle(vehicle: &Vehicle) {
    let color = DARKBLUE;
    let rotation = 45.0 + vehicle.angle;
    draw_poly(vehicle.position.x, vehicle.position.y, 4, VEHICLE_RADIUS, rotation, color);
}
