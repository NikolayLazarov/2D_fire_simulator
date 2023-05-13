use bevy::prelude::*;
use bevy_egui::egui::{vec2, Ui};
use bevy_egui::{egui, EguiContext};
use std::{thread, time};
// use crate::mat_coords;

use crate::fluid::N;
use crate::fluid::{self, FluidMatrix};
use crate::material_coords::Coords;
use crate::ui_state::{self};
use crate::Materials;
use crate::{element_changability, material_coords};

fn create_rect(
    ui: &mut Ui,
    r: u8,
    g: u8,
    b: u8,
    windows: &mut ResMut<element_changability::ElementChangebilityContext>,
    object_flag: bool,
) -> bool {
    let (rect, response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::click());
    ui.painter().rect(
        rect,
        0.0,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );
    if response.clicked() && object_flag {
        windows.side_panel_modify = true;
        true
    } else {
        false
    }
}

fn create_fire_in_range(
    d: f32,
    windows: &mut ResMut<element_changability::ElementChangebilityContext>,
    ui: &mut Ui,
    fluid: Mut<FluidMatrix>,
) {
    let mut red = d as u8;
    let mut yellow = d as u8;

    if d > 0.1 {
        red = 255;
        yellow = 255 - (d * 255.) as u8;
    }

    if create_rect(ui, red, yellow, 0, windows, true) {
        windows.fluid_for_change = fluid.clone();
        windows.fire_change_flag = true;
    }
}

fn check_if_material_at_position(
    x_cord: u32,
    y_cord: u32,
    x_material: u32,
    y_material: u32,
) -> bool {
    if x_cord == x_material && y_cord == y_material {
        return true;
    }
    return false;
}

// fn remove_material(
//     x: u32,
//     y: u32,
//     materials: Query<(Entity, &Materials), With<Materials>>,
//     commands: Commands,
// ) {
//     for (entity, material) in materials.iter() {
//         if x == material.position_x && y == material.position_y {}
//     }
// }

fn get_material_coeficient(density: &Vec<f32>, x: u32, y: u32) -> f32 {
    //gets the values of the cells around it
    let [mut left, mut right, mut up, mut down] = [0.; 4];
    if x as i32 - 1 > 0 {
        left = density[fluid::ix(x - 1, y) as usize];
    }
    if x + 1 < N - 1 {
        right = density[fluid::ix(x + 1, y) as usize];
    }
    if y as i32 - 1 > 0 {
        up = density[fluid::ix(x, y - 1) as usize];
    }
    if y + 1 < N - 1 {
        down = density[fluid::ix(x, y + 1) as usize];
    }
    left + right + up + down
}

fn render_density(
    ui: &mut Ui,
    density: &Vec<f32>,
    mut query_materials: Query<&mut Materials>,
    frames: u32,
    mut fluids: Query<&mut FluidMatrix>,
    windows: &mut ResMut<element_changability::ElementChangebilityContext>,
    query_materials_with_eintities: &Query<(Entity, &Materials), Without<Materials>>,
    mut commands: Commands,
    materials_coords: &mut ResMut<material_coords::CoordsList>,
) {
    for i in 0..N - 1 {
        ui.horizontal_top(|ui| {
            for j in 0..N - 1 {
                let x: u32 = i;
                let y: u32 = j;
                let d = density[fluid::ix(x, y) as usize];
            
                let mut material_flag: bool = false;
                // let no_material_flag: bool = false;
                let mut fluid_flag: bool = false;

                for mut material in query_materials.iter_mut() {
                    //checks whether there is a material in the given coords
                    if check_if_material_at_position(x, y, material.position_x, material.position_y)
                    {
                        //if there are frames and there is fuel to burn
                        if material.fuel > 0. && frames > 0 {
                            for mut fluid in fluids.iter_mut() {
                                let material_coeficient = get_material_coeficient(density, x, y);

                                //formula for decreasing fuel in the material
                                material.fuel -= material_coeficient
                                    * fluid.amount
                                    * (material.flammability as f32 / 100 as f32);

                                //updating state
                                let burn_power = fluid.amount;
                                let burn_speed_x = fluid.amount_x;
                                let burn_speed_y = fluid.amount_y;
                                fluid.add_density(
                                    material.position_x,
                                    material.position_y,
                                    burn_power,
                                );
                                fluid.add_velocity(
                                    material.position_x,
                                    material.position_y,
                                    burn_speed_x,
                                    burn_speed_y,
                                );
                            }
                        }

                        if material.fuel <= 0. {
                            // let mut cords_flag = false;
                            for _fluid in fluids.iter_mut() {
                                let coords = Coords {
                                    x: x,
                                    y: y,
                                    burned: false,
                                };

                                let coords_flag =
                                    materials_coords.material_coords.contains(&coords);

                                if coords_flag {
                                    let index = materials_coords
                                        .material_coords
                                        .iter()
                                        .position(|f| *f == coords)
                                        .unwrap();
                                    materials_coords.material_coords.remove(index);

                                      for (entity, material) in query_materials_with_eintities.iter()
                                    {
                                        if x == material.position_x && y == material.position_y {
                                            commands.entity(entity).despawn();
                                        }
                                    }
                                }
                            }
                            // material_flag = false;
                        } else {
                            let coeficient = material.fuel / 10.;
                            if create_rect(ui, 0, 255 - (coeficient as u8), 0, windows, true) {
                                windows.material_for_change = material.clone();
                                windows.material_change_flag = true;
                            }
                            material_flag = true;
                            break;
                        }
                    }
                    // else {
                    //     no_material_flag = true;
                    // }
                }
                if material_flag == true {
                    continue;
                }

                for fluid in fluids.iter_mut() {
                    if fluid_flag == false {
                        if x == fluid.fire_x && y == fluid.fire_y {
                            if create_rect(
                                ui,
                                255 as u8,
                                255 - (fluid.amount) as u8,
                                0,
                                windows,
                                true,
                            ) {
                                windows.fluid_for_change = fluid.clone();
                                windows.fire_change_flag = true;
                            }
                        } else {
                            if d > 0.5 {
                                //the fire
                                create_fire_in_range(d, windows, ui, fluid);
                            }
                            else if d > 0.01 && d < 0.5 {
                                //smoke
                                let (rect, _response) =
                                    ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
                                ui.painter().rect(
                                    rect,
                                    0.0,
                                    egui::Color32::from_gray((d * 255.0) as u8),
                                    
                                    egui::Stroke::new(
                                        9.0,
                                        egui::Color32::from_gray((d * 255.0) as u8), 
                                    ),
                                );
                            } else {
                                continue;
                            }
                        }
                        fluid_flag = true;
                        continue;
                    }
                }
                if fluid_flag == true {
                    continue;
                }
   
                if !(d > 1.0 || d < 0.) {
                     //the empty scene before and during simulation
                     let (rect, _response) =
                     ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
                 ui.painter().rect(
                     rect,
                     0.0,
                     egui::Color32::from_gray(d as u8),
                     egui::Stroke::new(9.0, egui::Color32::from_gray((d * 100.0) as u8)),
                 );
                }
            }
        });
    }
}

pub fn fluid_sys(
    query_fluid: Query<&mut FluidMatrix>,
    query_materials: Query<&mut Materials>,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<ui_state::UiState>,
    mut windows: ResMut<element_changability::ElementChangebilityContext>,
    query_materials_with_eintities: Query<(Entity, &Materials), Without<Materials>>,
    commands: Commands,
    mut materials_coordinates: ResMut<material_coords::CoordsList>,
) {
    let ten_millis = time::Duration::from_millis(200);

    let now = time::Instant::now();
    let mut frames = 0;

    if ui_state.start_simulation {
        frames = ui_state.fluid.frames;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        egui::Area::new("Fluid").show(ui.ctx(), |ui| {
            ui.label("Scene");

            if frames > 0 {
                ui_state.new_fluid = false;
                let fluid_x: u32 = ui_state.fluid.fire_x;
                let fluid_y: u32 = ui_state.fluid.fire_y;
                let amount: f32 = ui_state.fluid.amount;
                let amount_x: f32 = ui_state.fluid.amount_x;
                let amount_y: f32 = ui_state.fluid.amount_y;

                // if frames == 20 {
                ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                ui_state
                    .fluid
                    .add_velocity(fluid_x, fluid_y, amount_x, amount_y);

                // }
                ui_state.fluid.step(&mut materials_coordinates);
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                ui_state.fluid.frames -= 1;
                if ui_state.fluid.frames == 0 {
                    ui_state.start_simulation = false;
                }
            }
            render_density(
                ui,
                ui_state.fluid.get_density(),
                query_materials,
                frames,
                query_fluid,
                &mut windows,
                &query_materials_with_eintities,
                commands,
                &mut materials_coordinates,
            );
        });
    });
}
