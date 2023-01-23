use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{
    egui::{self, pos2},
    EguiPlugin,
};
mod Fluid;
mod UiState;
mod startup_systems;
mod systems;

use crate::Fluid::N;
fn main() {
    App::new()
        .init_resource::<UiState::UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(startup_systems::configure_visuals)
        .add_startup_system(startup_systems::configure_ui_state)
        .add_system(UiState::ui_state)
        .add_system(systems::fluid_sys)
        .run();
}

#[derive(Component, Debug, Clone)]
pub struct Material {
    name_type: String,
    flamability: f32,
    color: Color,
    width: f32,
    height: f32,
    position_x: f32,
    position_y: f32,
    clicked: bool,
    fuel: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flamability: 0.5,
            color: Color::BEIGE,
            width: 4.0,
            height: 5.0,
            position_x: 5.0,
            position_y: 5.0,
            clicked: false,
            fuel: 1000.0,
        }
    }
}
