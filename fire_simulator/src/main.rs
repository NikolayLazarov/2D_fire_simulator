use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{
    egui::{self, pos2},
    EguiPlugin,
};
mod Fluid;
mod MaterialChangability;
mod UiState;
mod startup_systems;
mod systems;

pub static mut materials_list: Vec<(u32, u32)> = vec![];

use crate::Fluid::N;
fn main() {
    App::new()
        .init_resource::<UiState::UiState>()
        .init_resource::<MaterialChangability::MaterialChangebility>()
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
    flammability: u32,
    width: f32,
    height: f32,
    position_x: u32,
    position_y: u32,
    fuel: f32,
    // entity_id: Entity,
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flammability: 50,
            width: 4.0,
            height: 5.0,
            position_x: 5,
            position_y: 5,
            fuel: 1000.0,
            // entity_id: Entity::index(Entity),
        }
    }
}
