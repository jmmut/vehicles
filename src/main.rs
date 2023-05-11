mod assertions;
mod draw;
mod gene;
mod light;
mod vehicle;

use crate::draw::{draw_light, draw_scene, draw_vehicle};
use crate::gene::{Coefficient, Crossed, Gene, Side};
use crate::light::Light;
use crate::vehicle::{advance_vehicle, stimulate, Vehicle, VEHICLE_RADIUS};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

const DEFAULT_WINDOW_TITLE: &'static str = "Braitenberg Vehicles";
const DEFAULT_WINDOW_WIDTH: i32 = 1280;
const DEFAULT_WINDOW_HEIGHT: i32 = 720;

#[macroquad::main(window_conf)]
async fn main() {
    let mut vehicles = reset_vehicles();
    let lights = vec![Light {
        position: Vec2::new(screen_width() * 3.0 / 4.0, screen_height() * 0.4),
        radius: screen_height() / 2.0,
    }];
    let mut advancing = true;
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            vehicles = reset_vehicles();
        }

        if is_key_pressed(KeyCode::Space) {
            advancing = !advancing;
        }
        if is_mouse_button_down(MouseButton::Left) {
            for vehicle in &vehicles {
                let (x, y) = mouse_position();
                let mouse_to_vehicle = vehicle.position - Vec2::new(x, y);
                let squared_distance = mouse_to_vehicle.dot(mouse_to_vehicle);
                if squared_distance < VEHICLE_RADIUS * VEHICLE_RADIUS {
                    let mut lines = vec![];
                    for gene in &vehicle.genes {
                        lines.push(format!("{:?}", gene));
                    }
                    lines.push(format!("left engine: {}, right engine {}, pos: {}, angle: {}",
                               vehicle.left_engine_activation,
                               vehicle.right_engine_activation,
                               vehicle.position,
                               vehicle.angle));
                    for line in lines {
                        widgets::Label::new(line).ui(&mut root_ui())
                    }
                }
            }
        }

        if advancing {
            for vehicle in &mut vehicles {
                stimulate(vehicle, &lights);
                advance_vehicle(vehicle);
                toroid_map(vehicle);
            }
        }
        draw_scene(&mut vehicles, &lights);
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

fn reset_vehicles() -> Vec<Vehicle> {
    let get_genes = |crossed, coefficient| {
        vec![
            Gene::new(crossed, Side::Left, coefficient),
            Gene::new(crossed, Side::Right, coefficient),
        ]
    };
    vec![
        Vehicle::new(
            get_genes(Crossed::Straight, Coefficient::Excitatory),
            Vec2::new(screen_width(), screen_height()) * 0.5,
            0.0,
        ),
        Vehicle::new(
            get_genes(Crossed::Straight, Coefficient::Inhibitory),
            Vec2::new(screen_width(), screen_height()) * 0.5,
            0.0,
        ),
        Vehicle::new(
            get_genes(Crossed::Crossed, Coefficient::Excitatory),
            Vec2::new(screen_width(), screen_height()) * 0.5,
            0.0,
        ),
        Vehicle::new(
            get_genes(Crossed::Crossed, Coefficient::Inhibitory),
            Vec2::new(screen_width(), screen_height()) * 0.5,
            0.0,
        ),
    ]
}

fn toroid_map(vehicle: &mut Vehicle) {
    let width = screen_width();
    let height = screen_height();
    while vehicle.position.x > width {
        vehicle.position.x -= width;
    }
    while vehicle.position.x < 0.0 {
        vehicle.position.x += width;
    }
    while vehicle.position.y > height {
        vehicle.position.y -= height;
    }
    while vehicle.position.y < 0.0 {
        vehicle.position.y += height;
    }
}
