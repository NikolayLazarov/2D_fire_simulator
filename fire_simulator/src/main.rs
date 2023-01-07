use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{
    egui::{self, pos2},
    EguiContext, EguiPlugin,
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
        // .add_startup_system(setup)
        .add_system(UiState::ui_state)
        // .add_system(all_elements_system)
        .add_system(systems::collision_system)
        // .add_system(systems::fluid_sys)
        // .add_system(material_fetch_system)
        // .add_system(fire_fetch_system)
        .run();
}

#[derive(Debug, Component, Clone)]
pub struct Fire {
    name: String,
    width: f32,
    height: f32,
    position_x: f32,
    position_y: f32,
    speed: f32,
    range: f32,
    direction: String,
    clicked: bool,
}

impl Default for Fire {
    fn default() -> Self {
        Self {
            name: String::from("Fire_1"),
            width: 4.0,
            height: 4.0,
            position_x: 4.0,
            position_y: 4.0,
            speed: 5.0,
            range: 2.0,
            direction: String::from("UP"),
            clicked: false,
        }
    }
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
        }
    }
}

// commands.spawn(Camera2dBundle::default());

// ui.add(
//     // SpriteBundle {
//     //     sprite: Sprite {
//     //         color: Color::rgb(0.25, 0.25, 0.75),
//     //         custom_size: Some(Vec2::new(50.0, 100.0)),
//     //         ..default()
//     //     },
//     //     ..default()
//     // }
// );
// commands.spawn(SpriteBundle {
//     sprite: Sprite {
//         color: Color::rgb(0.25, 0.25, 0.75),
//         custom_size: Some(Vec2::new(50.0, 100.0)),
//         ..default()
//     },
//     ..default()
// });

// fn setup(mut commands: Commands,){
//     commands.spawn(Camera2dBundle::default());

//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0.25, 0.25, 0.75),
//             custom_size: Some(Vec2::new(50.0, 100.0)),
//             ..default()
//         },
//         ..default()
//     });

// }
