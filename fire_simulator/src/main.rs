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
mod Windows;
mod startup_systems;
mod systems;

pub static mut materials_list: Vec<(u32, u32)> = vec![];

use crate::Fluid::N;
fn main() {
    App::new()
        .init_resource::<UiState::UiState>()
        .init_resource::<Windows::Windows>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(startup_systems::configure_visuals)
        .add_startup_system(startup_systems::configure_ui_state)
        .add_startup_system(startup_systems::configure_windows)
        .add_system(UiState::ui_state)
        .add_system(systems::fluid_sys)
        .run();
}

#[derive(Component, Debug, Clone)]
pub struct Materials {
    name_type: String,
    flamability: f32,
    color: Color,
    width: f32,
    height: f32,
    position_x: u32,
    position_y: u32,
    clicked: bool,
    fuel: f32,
    // entity_id: Entity,
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flamability: 0.5,
            color: Color::BEIGE,
            width: 4.0,
            height: 5.0,
            position_x: 5,
            position_y: 5,
            clicked: false,
            fuel: 600.0,
            // entity_id: Entity::index(Entity),
        }
    }
}
