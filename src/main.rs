mod assertions;
mod draw;
mod gene;
mod light;
mod math;
mod vehicle;

use crate::draw::draw_scene;
use crate::gene::{Coefficient, Crossed, Gene, Side};
use crate::light::Light;
use crate::math::closer_than;
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
    let mut showing_help = false;
    let mut exit = false;
    while !exit {
        handle_commands(&mut vehicles, &mut advancing, &mut showing_help, &mut exit);
        if advancing {
            for vehicle in &mut vehicles {
                stimulate(vehicle, &lights);
                advance_vehicle(vehicle);
                toroid_map(vehicle);
            }
        }
        draw_scene(&vehicles, &lights);
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

fn handle_commands(vehicles: &mut Vec<Vehicle>, advancing: &mut bool, showing_help: &mut bool, exit: &mut bool) {
    root_ui().label(None, "Press H to show/hide help");
    root_ui().label(None, "");
    if is_key_pressed(KeyCode::H) {
        *showing_help = !*showing_help;
    }
    if *showing_help {
        show_help();
    }
    if is_key_pressed(KeyCode::Escape) {
        *exit = true;
    }
    if is_key_pressed(KeyCode::R) {
        *vehicles = reset_vehicles();
    }
    if is_key_pressed(KeyCode::Space) {
        *advancing = !*advancing;
    }
    if is_mouse_button_down(MouseButton::Left) {
        show_vehicle_info_if_clicked(vehicles, mouse_position());
    }
}

fn show_vehicle_info_if_clicked(vehicles: &Vec<Vehicle>, mouse_position: (f32, f32)) {
    let (x, y) = mouse_position;
    let mouse_position = Vec2::new(x, y);
    for vehicle in vehicles {
        if closer_than(mouse_position, vehicle.position, VEHICLE_RADIUS) {
            for line in vehicle.to_strings() {
                widgets::Label::new(line).ui(&mut root_ui())
            }
        }
    }
}

fn show_help() {
    root_ui().label(None, "This program simulates simple vehicles in Valentino Braitenberg's book 'Vehicles'");
    root_ui().label(None, "");
    root_ui().label(None, "Press Escape to close");
    root_ui().label(None, "Press R to reset vehicle positions");
    root_ui().label(None, "Press Space to pause time");
    root_ui().label(None, "Click on any vehicle (square) to see its configuration (best done paused)");
    root_ui().label(None, "");
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
