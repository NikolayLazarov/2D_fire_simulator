use crate::Fire;
use crate::Fluid;
use crate::Fluid::N;
use crate::Material;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, pos2},
    EguiContext, EguiPlugin,
};

#[derive(Default, Resource)]
pub struct UiState {
    pub is_window_open: bool,
    pub material_window: bool,
    pub fire_window: bool,
    pub fluid_window: bool,
    pub new_material: bool,
    pub new_fire: bool,
    pub material: Material,
    pub fire: Fire,
    pub fluid: Fluid::FluidMatrix,
}

pub fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
) {
    let mut new_material_button = false;
    let mut new_fire_button = false;
    let mut material_button = false;
    let mut fire_button = false;
    let mut fluid_button = false;

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                material_button = ui.button("Material").clicked();
                fire_button = ui.button("Fire").clicked();
                fluid_button = ui.button("Update_fluid").clicked();
            });

            if ui_state.material_window {
                ui.heading("Material");

                ui.horizontal(|ui| {
                    ui.label("Your material: ");
                    ui.text_edit_singleline(&mut ui_state.material.name_type);
                });

                ui.add(egui::Slider::new(&mut ui_state.material.width, 0.0..=30.0).text("Width"));
                if ui.button("Increment").clicked() {
                    ui_state.material.width += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.material.height, 0.0..=30.0).text("Height"));
                if ui.button("Increment").clicked() {
                    ui_state.material.height += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.material.position_x, 0.0..=30.0).text("X axys"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.material.position_x += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.material.position_y, 0.0..=30.0).text("Y axys"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.material.position_y += 1.0;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("addDensity");
                    new_material_button = ui.button("New").clicked();
                });
            } else if ui_state.fire_window {
                ui.horizontal(|ui| {
                    ui.label("Your Fire: ");
                    ui.text_edit_singleline(&mut ui_state.fire.name);
                });

                ui.add(egui::Slider::new(&mut ui_state.fire.width, 0.0..=30.0).text("Width"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.width += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.height, 0.0..=30.0).text("Height"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.height += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.position_x, 0.0..=30.0).text("X axys"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.position_x += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.position_y, 0.0..=30.0).text("Y axys"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.position_y += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.speed, 0.0..=30.0).text("Speed"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.speed += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fire.range, 0.0..=30.0).text("Range"));
                if ui.button("Increment").clicked() {
                    ui_state.fire.range += 1.0;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("New");
                    new_fire_button = ui.button("New").clicked();
                });
            } else if ui_state.fluid_window {
                ui.label("Change fluid");

                ui.separator();

                ui.add(egui::Slider::new(&mut ui_state.fluid.fluid_y, 0..=N - 2).text("Fluid X"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fluid_x += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.fluid_x, 0..=N - 2).text("Fluid Y"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fluid_y += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.amount, 0.0..=30.0).text("Power"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_x, 0.0..=30.0).text("Velocity X"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_x += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_y, 0.0..=30.0).text("Velocity Y"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_y += 1.0;
                }

                ui.horizontal(|ui| {
                    ui.label("Add density");
                    let mut update_fluid_density = ui.button("Add Density").clicked();

                    if update_fluid_density {
                        //temporary changes
                        println!("here");

                        let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                        let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                        let mut amount: f32 = ui_state.fluid.amount;
                        let mut amount_x: f32 = ui_state.fluid.amount_x;
                        let mut amount_y: f32 = ui_state.fluid.amount_y;

                        ui_state.fluid.add_density(fluid_x, fluid_y, amount);

                        //example

                        ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
                        ui_state.fluid.step();
                        for i in 0..N - 1 {
                            for j in 0..N - 1 {
                                let x: u32 = i;
                                let y: u32 = j;
                                let d = ui_state.fluid.get_density()[Fluid::IX(x, y) as usize];
                                print!("{} ", d);
                                //use d as alpha color a
                                //no stroke
                                //square(x,y, Scale)
                            }
                            println!();
                        }
                    }
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
        ui_state.fluid_window = false;
        ui_state.fire_window = false;
        ui_state.material_window = true;
    }

    if fire_button {
        ui_state.fluid_window = false;
        ui_state.material_window = false;
        ui_state.fire_window = true;
    }

    if fluid_button {
        ui_state.material_window = false;
        ui_state.fire_window = false;
        ui_state.fluid_window = true;
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
