use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::fluid::N;
mod element_changability;
mod fire_window;
mod fluid;
mod material_window;
mod startup_systems;
mod systems;
mod ui_state;
mod material_coords;
mod Fluid;

// pub static mut mat_coords: material_coords::CoordsList =  material_coords::CoordsList::new();
fn main() {

    

    App::new()
        .init_resource::<material_coords::CoordsList>()
        .init_resource::<ui_state::UiState>()
        .init_resource::<element_changability::ElementChangebilityContext>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(startup_systems::configure_visuals)
        .add_startup_system(startup_systems::configure_ui_state)
        .add_startup_system(startup_systems::configure_windows)
        .add_system(ui_state::ui_state)
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
    size: u32,
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            name_material: String::from("Material"),
            flammability: 50,
            position_x: 5,
            position_y: 5,
            fuel: 1000.0,
            size: 1,
        }
    }
}

pub fn create_shape(material: Materials) -> Vec<Materials> {
    let x = material.position_x as i32;
    let y = material.position_y as i32;
    let size = material.size as i32;
    let range_x_left = x - (size / 2);
    let range_x_right = x + (size / 2);
    let range_y_up = y - (size / 2);
    let range_y_down = y + (size / 2);

    let mut material_parts: Vec<Materials> = vec![];
    for i in range_x_left..=range_x_right {
        for j in range_y_up..=range_y_down {
            //if
            if i <= 0 || j <= 0 || j >= (N - 1) as i32 || i >= (N - 1) as i32 {
                continue;
            }
            if size % 2 == 0
                && (i == range_x_left || j == range_y_up)
            {
                continue;
            }

            material_parts.push(Materials {
                name_material: material.name_material.clone(),
                flammability: material.flammability,
                position_x: i as u32,
                position_y: j as u32,
                fuel: material.fuel,
                size: 1,
            });
        }
    }

    return material_parts;
}

