use crate::element_changability;
use crate::fluid;
use crate::fluid::N;
use crate::Materials;
use bevy::prelude::*;
use bevy_egui::{EguiContext};
use bevy_egui::egui;

#[derive(Default, Resource)]
pub struct UiState {
    pub is_window_open: bool,
    pub material_window: bool,
    pub fire_window: bool,
    pub fluid_window: bool,
    pub new_material: bool,
    pub new_fluid: bool,
    pub material: Materials,
    pub fluid: fluid::FluidMatrix,
    pub created_fire: bool,
    pub window_change_materials: bool,
    pub start_simulation: bool,
    pub restart_simulation: bool,
    pub counter_fire_size: u32,
}

pub fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut windows: ResMut<element_changability::ElementChangebilityContext>,
    mut query_fire_entity: Query<Entity, With<fluid::FluidMatrix>>,
    mut query_materials: Query<(Entity, &Materials), With<Materials>>,
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
                ui.heading("Change Window");
                if windows.material_change_flag {

                    for (entity, transform) in query_materials.iter() {
                        if transform.position_x == windows.material_for_change.position_x && 
                        transform.position_y == windows.material_for_change.position_y  {
                            commands.entity(entity)
                                .remove::<Materials>();
                        }
                    }
                            let mut change_material_button = false;
                    ui.heading("Material");

                    ui.horizontal(|ui| {
                        ui.label("Your material: ");
                        ui.text_edit_singleline(&mut windows.material_for_change.name_material);
                    });

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

                    ui.add(
                        egui::Slider::new(&mut windows.material_for_change.flammability, 0..=100)
                            .text("Flammability"),
                    );
                    if ui.button("Increment").clicked() {
                        ui_state.material.flammability += 1;
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Change Material");
                        change_material_button = ui.button("Change").clicked();
                    });

                    if change_material_button == true {
                       
                        commands.spawn(windows.material_for_change.clone());
                    
                        close_window = true;
                        windows.material_change_flag = false;
                    }
                }
                if windows.fire_change_flag {
                    ui.heading("Fire");

                    ui.separator();

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.delta_time, 0.0..=3.0)
                            .text("Timestep"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.delta_time += 0.1;
                    }
                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.diffusion, 0.0..=10.0)
                            .text("Diffusion"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.diffusion += 0.001;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.viscosity, 0.0..=1.0)
                            .text("Viscosity"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.viscosity += 0.0000001;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.fire_x, 0..=N - 2)
                            .text("Fire Y"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.fire_x += 1;
                    }
                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.fire_y, 0..=N - 2)
                            .text("Fire X"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.fire_y += 1;
                    }
                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount, 0.0..=255.0)
                            .text("Power"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.amount += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount_x, 0.0..=200.0)
                            .text("Velocity Y"),
                    );
                    if ui.button("Increment").clicked() {
                        windows.fluid_for_change.amount_x += 1.0;
                    }

                    ui.add(
                        egui::Slider::new(&mut windows.fluid_for_change.amount_y, 0.0..=200.0)
                            .text("Velocity X"),
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
                        ui.label("Change Fire");
                        let change_fire = ui.button("Change").clicked();

                        if change_fire == true {
                            for fire in query_fire_entity.iter() {
                                commands.entity(fire).despawn();
                            }
                            ui_state.fluid = windows.fluid_for_change.clone();

                            commands.spawn(windows.fluid_for_change.clone());
                            close_window = true;
                            windows.fire_change_flag = false;
                        }

                    });
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.allocate_space(ui.available_size());
                });
            });
        if close_window {
            windows.side_panel_modify = false;
        }
    }

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                material_button = ui.button("Material").clicked();
                fluid_button = ui.button("Fire").clicked();
            });

            if ui_state.material_window {
                ui.heading("Material");

                ui.horizontal(|ui| {
                    ui.label("Your material: ");
                    ui.text_edit_singleline(&mut ui_state.material.name_material);
                });

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

                ui.add(
                    egui::Slider::new(&mut ui_state.material.flammability, 0..=100)
                        .text("Flammability"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.material.flammability += 1;
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Add Material");
                    new_material_button = ui.button("New").clicked();
                });
            } else if ui_state.fluid_window {
                ui.heading("Fire");

                ui.separator();

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.delta_time, 0.0..=3.0).text("Timestep"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.delta_time += 0.1;
                }
                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.diffusion, 0.0..=10.0).text("Diffusion"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.diffusion += 0.001;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.viscosity, 0.0..=1.0).text("Viscosity"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.viscosity += 0.0000001;
                }

                ui.add(egui::Slider::new(&mut ui_state.fluid.fire_x, 1..=N - 2).text("Fire Y"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fire_x += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.fire_y, 1..=N - 2).text("Fire X"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.fire_y += 1;
                }
                ui.add(egui::Slider::new(&mut ui_state.fluid.amount, 0.0..=255.0).text("Power"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_x, -200.0..=200.0)
                        .text("Velocity Y"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_x += 1.0;
                }

                ui.add(
                    egui::Slider::new(&mut ui_state.fluid.amount_y, -200.0..=200.0)
                        .text("Velocity X"),
                );
                if ui.button("Increment").clicked() {
                    ui_state.fluid.amount_y += 1.0;
                }

                ui.add(egui::Slider::new(&mut ui_state.fluid.frames, 0..=100).text("Frames"));
                if ui.button("Increment").clicked() {
                    ui_state.fluid.frames += 1;
                }

                ui.horizontal(|ui| {
                    ui.label("Add Fire");
                    let  update_fluid_density = ui.button("New").clicked();

                    if update_fluid_density {
                        ui_state.new_fluid = true;
                    }
                });
            }
            ui.horizontal(|ui| {
                if ui.button("Start simulation").clicked() && ui_state.created_fire {
                    ui_state.start_simulation = true;
                }

                if ui.button("Restart Scene").clicked() {
                    ui_state.restart_simulation = true;
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.allocate_space(ui.available_size());
            });
        });

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |_ui| {
        
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(egui_ctx.ctx_mut(), |_ui| {
        
    });

    if new_material_button {
        ui_state.new_material = true;
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
        let x = ui_state.material.position_x;
        let y = ui_state.material.position_y;

        if !ui_state.fluid.materials_coords.contains(&(x, y)) {
             commands.spawn(ui_state.material.clone());
            ui_state.fluid.materials_coords.push((x, y));
            }
        ui_state.new_material = false;
    }
    if ui_state.new_fluid && !ui_state.created_fire
    /*||ui_state.restart_simulation*/
    {
        if ui_state.created_fire {
            for fire in &mut query_fire_entity {
                commands.entity(fire).despawn();
            }
        }

        ui_state.new_fluid = false;
        
        ui_state.created_fire = true;
        commands.spawn(ui_state.fluid.clone());
    }

    if ui_state.restart_simulation {
        if ui_state.created_fire {
            for fire in &mut query_fire_entity {
                commands.entity(fire).despawn();
            }
        }

        for (entity,_material)  in &mut query_materials{
            commands.entity(entity).despawn();
        }
        ui_state.new_fluid = false;
        ui_state.fluid = fluid::FluidMatrix::new();
        ui_state.created_fire = false;
        ui_state.restart_simulation = false;
        ui_state.start_simulation = false;
        }
}
