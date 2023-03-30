use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod element_changability;
mod fire_window;
mod fluid;
mod material_window;
mod startup_systems;
mod systems;
mod ui_state;

pub static mut MATERIALS_LIST: Vec<(u32, u32)> = vec![];

fn main() {
    App::new()
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
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            name_material: String::from("Material"),
            flammability: 50,
            position_x: 5,
            position_y: 5,
            fuel: 1000.0,
        }
    }
}
