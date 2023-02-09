// use crate::Fire;
use crate::Fluid;
use crate::Fluid::N;
use crate::Materials;
use crate::Windows;
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::{
    egui::{self, pos2},
    EguiContext, EguiPlugin,
};

use crate::materials_list;

#[derive(Default, Resource)]
pub struct UiState {
    pub is_window_open: bool,
    pub material_window: bool,
    pub fire_window: bool,
    pub fluid_window: bool,
    pub new_material: bool,
    pub new_fire: bool,
    pub new_fluid: bool,
    pub material: Materials,
    pub fluid: Fluid::FluidMatrix,
    pub start_simulation: bool,

    pub window_change_materials: bool,
}

pub fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut windows: ResMut<Windows::Windows>,
) {
    let mut new_material_button = false;
    let mut material_button = false;
    let mut fluid_button = false;

    if windows.side_panel_modify {
        let mut close_window = false;
        egui::SidePanel::right("Side_panel_for_modifications")
            .default_width(200.0)
            .resizable(true)
            .show(egui_ctx.ctx_mut(), |ui| {
                if windows.material_change_flag {
                    // let material = & windows.material_for_change;
                    // ui.label(format!("Material =  {}",  material.name_type));
                    // ui.label(format!("Material x =  {}",  material.position_x));
                    // ui.label(format!("Material y =  {}",  material.position_y));
                    // ui.label(format!("Material flamability =  {}",  material.flamability));

                    ////
                    let mut change_material_button = false;
                    ui.heading("Material");

                    ui.horizontal(|ui| {
                        ui.label("Your material: ");
                        ui.text_edit_singleline(&mut windows.material_for_change.name_type);
                    });

                    ui.add(
                        egui::Slider::new(&mut windows.material_for_change.width, 0.0..=30.0)
                            .text("Width"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.material_for_change.width += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.material_for_change.height, 0.0..=30.0)
                            .text("Height"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.material_for_change.height += 1.0;
                    }
                    //do not why x and y are swapped so I Swapp them
                    ui.add(
                        egui::Slider::new(&mut windows.material_for_change.position_y, 0..=N - 1)
                            .text("X axys"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.material_for_change.position_y += 1;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.material_for_change.position_x, 0..=N - 1)
                            .text("Y axys"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.material_for_change.position_x += 1;
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("addDensity");
                        change_material_button = ui.button("Change").clicked();
                    });

                    if change_material_button == true {
                        let mut vector = (
                            windows.material_for_change.position_x.clone(),
                            windows.material_for_change.position_y.clone(),
                        );
                        // materials_list.push(vector);
                        let entity = commands.spawn(windows.material_for_change.clone()).id();
                        windows.materials_entities.push(entity);
                        println!("Entity = {:?}", windows.materials_entities);

                        close_window = true;
                        windows.material_change_flag = false;
                    }

                    ////
                }
                if windows.fire_change_flag {
                    ui.label("Change fluid");

                    ui.separator();

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.fluid_x, 0..=N - 2)
                            .text("Fluid X"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.fluid_x += 1;
                    }
                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.fluid_y, 0..=N - 2)
                            .text("Fluid Y"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.fluid_y += 1;
                    }
                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount, 0.0..=200.0)
                            .text("Power"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.amount += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.fire_range, 0..=10)
                            .text("Range"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.fire_range += 1;
                        windows.fluid_for_change.counter_range +=
                            windows.fluid_for_change.fire_range
                                * windows.fluid_for_change.fire_range;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount_x, 0.0..=200.0)
                            .text("Velocity X"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.amount_x += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount_y, 0.0..=200.0)
                            .text("Velocity Y"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.amount_y += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.frames, 0..=100)
                            .text("Frames"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.frames += 1;
                    }

                    ui.horizontal(|ui| {
                        ui.label("Add density");
                        let mut change_fire = ui.button("Add Density").clicked();

                        if change_fire == true {
                            let entity = commands.spawn(windows.fluid_for_change.clone()).id();
                            close_window = true;
                            windows.fire_change_flag = false;
                        }

                        //    ui_state.start_simulation = true;
                        //  ui_state.new_fluid = true;
                        //update_fluid_density = false;
                    });
                }
                // close_window = ui.button("Change").clicked();

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.allocate_space(ui.available_size());
                });
            });
        if close_window {
            windows.side_panel_modify = false;
            // windows.fire_change_flag = false;
            // windows.material_change_flag = false;
        }
    }

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                material_button = ui.button("Material").clicked();
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
                //do not why x and y are swapped so I Swapp them
                ui.add(
                    egui::Slider::new(&mut ui_state.material.position_y, 0..=N - 1).text("X axys"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.material.position_y += 1;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.material.position_x, 0..=N - 1).text("Y axys"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.material.position_x += 1;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("addDensity");
                    new_material_button = ui.button("New").clicked();
                });
            } else if ui_state.fluid_window {
                ui.label("Change fluid");

                ui.separator();

                ui.add(egui::Slider::new(&mut ui_state.fluid.fluid_x, 0..=N - 2).text("Fluid X"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fluid_x += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.fluid_y, 0..=N - 2).text("Fluid Y"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fluid_y += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.amount, 0.0..=200.0).text("Power"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fluid.fire_range, 0..=10).text("Range"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fire_range += 1;
                    ui_state.fluid.counter_range +=
                        ui_state.fluid.fire_range * ui_state.fluid.fire_range;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_x, 0.0..=200.0).text("Velocity X"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_x += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_y, 0.0..=200.0).text("Velocity Y"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_y += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fluid.frames, 0..=100).text("Frames"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.frames += 1;
                }

                ui.horizontal(|ui| {
                    ui.label("Add density");
                    let mut update_fluid_density = ui.button("Add Density").clicked();

                    if update_fluid_density {
                        for i in 0..N - 1 {
                            for j in 0..N - 1 {
                                let x: u32 = i;
                                let y: u32 = j;
                                let d = ui_state.fluid.get_density()[Fluid::IX(x, y) as usize];
                                print!("{} ", d);
                            }
                            println!();
                        }
                        ui_state.new_fluid = true;
                        // update_fluid_density = false;
                    }
                });
            }

            if ui.button("start simulation").clicked() {
                ui_state.start_simulation = true;
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

    if material_button {
        ui_state.fire_window = false;
        ui_state.material_window = true;
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
    if ui_state.new_fluid {
        commands.spawn(ui_state.fluid.clone());
    }
}
