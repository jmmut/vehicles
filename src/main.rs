mod draw;
mod gene;
mod vehicle;
mod light;
mod assertions;

use macroquad::prelude::*;
use crate::gene::{Coefficient, Gene, Side};
use crate::vehicle::{advance_vehicle, stimulate, Vehicle};
use crate::draw::{draw_light, draw_vehicle};
use crate::light::Light;

const DEFAULT_WINDOW_TITLE: &'static str = "Braitenberg Vehicles";
const DEFAULT_WINDOW_WIDTH: i32 = 1280;
const DEFAULT_WINDOW_HEIGHT: i32 = 720;

#[macroquad::main(window_conf)]
async fn main() {
    let mut vehicles = vec![Vehicle::new(
        vec![Gene {
            sensor_side: Side::Left,
            coefficient: Coefficient::Excitatory,
            motor_connection: Side::Left,
        }],
        Vec2::new(screen_width(), screen_height()) * 0.5,
        0.0,
    )];

    let lights = vec![Light {position: Vec2::new(screen_width() *3.0 / 4.0, screen_height() *0.4), radius: screen_height()/2.0}];
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        for vehicle in &mut vehicles {
            stimulate(vehicle, &lights);
            advance_vehicle(vehicle);
        }
        clear_background(SKYBLUE);
        for light in &lights {
            draw_light(light);
        }
        for vehicle in &vehicles {
            draw_vehicle(vehicle);
        }
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}
