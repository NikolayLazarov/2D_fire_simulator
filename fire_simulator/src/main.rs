use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{
    egui::{self, pos2},
    EguiPlugin,
};
mod ElementChangability;
mod Fluid;
mod UiState;
mod startup_systems;
mod systems;

pub static mut materials_list: Vec<(u32, u32)> = vec![];

use crate::Fluid::N;
fn main() {
    App::new()
        .init_resource::<UiState::UiState>()
        .init_resource::<ElementChangability::ElementChangebilityContext>()
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
    name_material: String,
    flammability: u32,
    position_x: u32,
    position_y: u32,
    fuel: f32,
    material_entity: Vec<Entity>,
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            name_material: String::from("Material"),
            flammability: 50,
            position_x: 5,
            position_y: 5,
            fuel: 1000.0,
            material_entity: vec![],
        }
    }
}
