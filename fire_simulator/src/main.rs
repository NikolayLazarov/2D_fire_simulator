use std::fmt::format;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::{
    egui::{self, pos2},
    EguiContext, EguiPlugin,
};

fn material_fetch_system(
    mut query: Query<&Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Central Area").show(ui.ctx(), |ui| {
            for mut material in &mut query {
                let mut button = false;
                ui.label(format!(
                    "Material = {}, width = {}, height = {} ",
                    material.name_type, material.width, material.height
                ));
                button = ui.button(material.name_type.to_string()).clicked();
            }
        });
    });
}

fn fire_fetch_system(
    mut query: Query<&Fire>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
) {
    egui::Area::new("Fires").show(egui_ctx.ctx_mut(), |ui| {
        for mut fire in &mut query {
            let mut button = false;
            ui.label(format!(
                "Fire = {}, range = {}, speed{}",
                fire.name, fire.range, fire.speed
            ));
            button = ui.button(fire.name.to_string()).clicked();
        }
    });
}

fn all_elements_system(
    mut query_fires: Query<&Fire>,
    mut query_materials: Query<&Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
){
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Central Area").show(ui.ctx(), |ui| {
            for mut material in &mut query_materials {
                let mut button = false;
                ui.label(format!(
                    "Material = {}, width = {}, height = {} ",
                    material.name_type, material.width, material.height
                ));
                button = ui.button(material.name_type.to_string()).clicked();
            }

            for mut fire in &mut query_fires {
                let mut button = false;
                ui.label(format!(
                    "Fire = {}, range = {}, speed{}",
                    fire.name, fire.range, fire.speed
                ));
                button = ui.button(fire.name.to_string()).clicked();
            }
        });
    });
}

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_visuals)
        .add_startup_system(configure_ui_state)
        // .add_startup_system(setup)
        .add_system(ui_state)
        .add_system(all_elements_system)
        // .add_system(material_fetch_system)
        // .add_system(fire_fetch_system)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    is_window_open: bool,
    material_window: bool,
    fire_window: bool,
    new_material: bool,
    new_fire: bool,
    material: Material,
    fire: Fire,
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
    ui_state.material = Material::default();
    ui_state.fire = Fire::default();
    ui_state.new_material = false;
    ui_state.new_fire = false;
    ui_state.material_window = false;
    ui_state.fire_window = false;
}

fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
) {
    let mut new_material_button = false;
    let mut new_fire_button = false;
    let mut material_button = false;
    let mut fire_button = false;
    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                material_button = ui.button("Material").clicked();
                fire_button = ui.button("Fire").clicked();
            });

            if ui_state.material_window {
                ui.heading("Material");

                ui.horizontal(|ui| {
                    ui.label("Your material: ");
                    ui.text_edit_singleline(&mut ui_state.material.name_type);
                });

                ui.add(egui::Slider::new(&mut ui_state.material.width, 0..=30).text("Width"));
                if ui.button("Increment").clicked() {
                    ui_state.material.width += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.material.height, 0..=30).text("Height"));
                if ui.button("Increment").clicked() {
                    ui_state.material.height += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.material.position_x, 0..=30).text("X axys"));
                if ui.button("Increment").clicked() {
                    ui_state.material.position_x += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.material.position_y, 0..=30).text("Y axys"));
                if ui.button("Increment").clicked() {
                    ui_state.material.position_y += 1;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("New");
                    new_material_button = ui.button("New").clicked();
                });
            } else if ui_state.fire_window {
                ui.horizontal(|ui| {
                    ui.label("Your Fire: ");
                    ui.text_edit_singleline(&mut ui_state.fire.name);
                });

                ui.add(egui::Slider::new(&mut ui_state.fire.width, 0..=30).text("Width"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.width += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.height, 0..=30).text("Height"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.height += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.position_x, 0..=30).text("X axys"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.position_x += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.position_y, 0..=30).text("Y axys"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.position_y += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.speed, 0..=30).text("Speed"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.speed += 1;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.range, 0..=30).text("Range"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.range += 1;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("New");
                    new_fire_button = ui.button("New").clicked();
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.allocate_space(ui.available_size());
            });
        });

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Top pannel");
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Bottom pannel");
    });

    if new_material_button {
        ui_state.new_material = !ui_state.new_material;
    }
    if new_fire_button {
        ui_state.new_fire = !ui_state.new_fire;
    }

    if material_button {
        ui_state.fire_window = false;
        ui_state.material_window = true;
    }

    if fire_button {
        ui_state.material_window = false;
        ui_state.fire_window = true;
    }

    if ui_state.new_material {
        commands.spawn(ui_state.material.clone());
        ui_state.new_material = false;
    }
    if ui_state.new_fire {
        commands.spawn(ui_state.fire.clone());
        ui_state.new_fire = false;
    }
}

#[derive(Debug, Component, Clone)]
struct Fire {
    name: String,
    width: u32,
    height: u32,
    position_x: u32,
    position_y: u32,
    speed: u32,
    range: u32,
    direction: String,
    clicked: bool,
}

impl Default for Fire {
    fn default() -> Self {
        Self {
            name: String::from("Fire_1"),
            width: 4,
            height: 4,
            position_x: 4,
            position_y: 4,
            speed: 5,
            range: 2,
            direction: String::from("UP"),
            clicked: false,
        }
    }
}

#[derive(Component, Debug, Clone)]
struct Material {
    name_type: String,
    flamability: f32,
    color: Color,
    width: u32,
    height: u32,
    position_x: u32,
    position_y: u32,
    clicked: bool,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flamability: 0.5,
            color: Color::BEIGE,
            width: 4,
            height: 5,
            position_x: 5,
            position_y: 5,
            clicked: false,
        }
    }
}
