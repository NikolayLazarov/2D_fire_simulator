use bevy::ecs::query;
use bevy::render::view::window;
use bevy::ui;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_egui::egui::epaint::RectShape;
use std::{thread, time};
// extern crate perlin_noise;
// use perlin::PerlinNoise;

use bevy_egui::egui::{vec2, Pos2, Rounding, Ui};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::ElementChangability;
use crate::Fluid::N;
use crate::Fluid::{self, FluidMatrix};
use crate::Materials;
use crate::UiState::{self, ui_state};

fn create_rect(
    ui: &mut Ui,
    r: u8,
    g: u8,
    b: u8,
    windows: &mut ResMut<ElementChangability::ElementChangebilityContext>,
    object_flag: bool,
) -> bool {
    let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::click());
    ui.painter().rect(
        rect,
        0.0,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );
    if Response.clicked() && object_flag {
        windows.side_panel_modify = true;
        true
    } else {
        false
    }
}

fn create_fire_in_range(
    d: f32,
    windows: &mut ResMut<ElementChangability::ElementChangebilityContext>,
    ui: &mut Ui,
    fluid: Mut<FluidMatrix>,
) {
    let mut red = d as u8;
    let mut yellow = d as u8;

    if d > 0.1 {
        red = 255;
        yellow = 255 - (d * 255.) as u8;
    }

    if create_rect(
        ui, red,
        //maybe here use only the amount -> see how much green makes orange or yellow
        yellow, 0, windows, true,
    ) {
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
fn check_if_in_range(
    x_cord: i32,
    y_cord: i32,
    x_material: i32,
    y_material: i32,
    mut range: i32,
    type_fire: u32,
) -> bool {
    let mut flag_x = false;
    let mut flag_y = false;
    if type_fire % 2 == 1 {
        let down_range = -range;
        if x_material - x_cord <= range && x_material - x_cord >= down_range {
            flag_x = true;
        }
        if y_material - y_cord <= range && y_material - y_cord >= down_range {
            flag_y = true;
        }

        flag_y && flag_x
    } else {
        range /= 2;
        if !(x_cord >= x_material - range
            && x_cord <= x_material + range
            && y_cord == y_material - range)
        {
            flag_x = true;
        }
        if !(y_cord >= y_material - range
            && y_cord <= y_material + range
            && x_cord == x_material - range)
        {
            flag_y = true;
        }
        flag_x && flag_y
    }
}

fn render_density(
    ui: &mut Ui,
    density: &Vec<f32>,
    mut query_materials: Query<&mut Materials>,
    mut commands: Commands,
    frames: u32,
    mut fluids: Query<&mut FluidMatrix>,
    windows: &mut ResMut<ElementChangability::ElementChangebilityContext>,
) {
    //x
    for i in 0..N - 1 {
        ui.horizontal_top(|ui| {
            //y
            for j in 0..N - 1 {
                let x: u32 = i;
                let y: u32 = j;
                let mut d = density[Fluid::IX(x, y) as usize];

                //flag if current cell is a material
                let mut material_flag: bool = false;
                //flag if current cell is a fluid
                let mut fluid_flag: bool = false;

                for mut material in query_materials.iter_mut() {
                    //checks if there is material at the given position
                    if check_if_material_at_position(x, y, material.position_x, material.position_y)
                    {
                        //if the material has not burned out yet and the simulation has started

                        if material.fuel > 0. && frames > 0 {
                            for mut fluid in fluids.iter_mut() {
                                let [mut left, mut right, mut up, mut down] = [0.; 4];
                                if x as i32 - 1 > 0 {
                                    left = density[Fluid::IX(x - 1, y) as usize];
                                }
                                if x + 1 < N - 1 {
                                    right = density[Fluid::IX(x + 1, y) as usize];
                                }
                                if y as i32 - 1 > 0 {
                                    up = density[Fluid::IX(x, y - 1) as usize];
                                }
                                if y + 1 < N - 1 {
                                    down = density[Fluid::IX(x, y + 1) as usize];
                                }
                                let mut material_coeficient = left + right + up + down;

                                println!(
                                    "d = {}, amount = {}, flamability = {}",
                                    material_coeficient, fluid.amount, material.flammability as f32
                                );
                                material.fuel -= material_coeficient
                                    * fluid.amount
                                    * (material.flammability as f32 / 100 as f32);
                                println!(
                                    "material fluid = {} and minus = {}",
                                    material.fuel,
                                    d * fluid.amount * (material.flammability as f32 / 100 as f32)
                                );
                                let burn_power = fluid.amount;
                                let burn_speed_x = fluid.amount_x;
                                let burn_speed_y = fluid.amount_y;
                                //not sure if this works
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
                                fluid.step();
                            }
                        }

                        //if the material has been burnt
                        if material.fuel <= 0. {
                            // let mut entity_ids ={}
                            let mut cords_flag = false;
                            for mut fluid in fluids.iter_mut() {
                                //check if material is still in the material list
                                cords_flag = fluid.materials_coords.contains(&(x, y));

                                //removes the material from the cords list
                                if cords_flag {
                                    //here should be a new black rect or one that is goig to be filled with something
                                    fluid.materials_coords.retain(|&f| f == (x, y));
                                }
                            }
                            //despawn
                            //
                            //false because it changes during the simulation
                            // create_rect(ui, (255 - d as u8), 0, 0, windows, false);
                            //should change this to be more efficient at some point
                            // create_rect(ui, 0, 0, 0, windows, false);
                            // break;
                        } else {
                            let coeficient = material.fuel / 10.;
                            // if material.fuel == 1000.{
                            //     if create_rect(ui, 139,69,19 , windows, true) {
                            //         windows.material_for_change = material.clone();
                            //         windows.material_change_flag = true;
                            //         // commands.entity(material).despawn();
                            //     }
                            // }else{
                            if create_rect(ui, 255, 255 - (coeficient as u8), 0, windows, true) {
                                windows.material_for_change = material.clone();
                                windows.material_change_flag = true;
                                // commands.entity(material).despawn();
                            }
                            // }
                        }
                        material_flag = true;
                    }
                }
                if material_flag == true {
                    continue;
                }

                for mut fluid in fluids.iter_mut() {
                    // if fluid.fluid_x == x && fluid.fluid_y == y && fluid_flag == false {
                    //     // println!("fluid loop");
                    //     // create_rect(ui, (255-d as u8), 0,0 );
                    //     create_rect(ui, (255 as u8), d as u8,0 );

                    //     fluid_flag = true;
                    // }
                    //rect // fluid

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
                                // fluid.counter_range = 1;
                            }
                            // continue;
                        } else {
                            if d >  0.2 {
                                // let perlin = PerlinNoise::new();
                                // println!("We are here {}", perlin.get(132.2) );
                                create_fire_in_range(d, windows, ui, fluid);
                            } else if d > 0.01 && d < 0.4 {
                                let (rect, Response) =
                        ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
                    ui.painter().rect(
                        rect,
                        0.0,
                        egui::Color32::from_gray(255 -(d * 255.0) as u8),
                        egui::Stroke::new(9.0, egui::Color32::from_gray(255- (d * 255.0) as u8)),
                    );
                            } else{
                                continue;
                            }
                        }
                        fluid_flag = true;
                        continue;
                        //         //see if this stops others from emiting
                    }
                }
                if fluid_flag == true {
                    continue;
                }

                if d > 255.0 || d < 0. {
                    //black for where there is not density or is over
                    d = 0.0;
                    create_rect(ui, d as u8, 0, 0, windows, false);
                } else {
                    let (rect, Response) =
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
    mut query_fluid: Query<&mut FluidMatrix>,
    mut query_materials: Query<&mut Materials>,

    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut ui_state: ResMut<UiState::UiState>,
    mut windows: ResMut<ElementChangability::ElementChangebilityContext>,
) {
    //let ten_millis = time::Duration::from_millis(1000 / 24);
    let ten_millis = time::Duration::from_millis(200);

    let now = time::Instant::now();
    let mut frames = 0;

    if ui_state.start_simulation {
        frames = ui_state.fluid.frames;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        // ui.label("Scene");

        egui::Area::new("Fluid").show(ui.ctx(), |ui| {
            ui.label("Scene");

            if frames > 0 {
                ui_state.new_fluid = false;
                // ui_state.start_simulation = true;
                let mut fluid_x: u32 = ui_state.fluid.fire_x;
                let mut fluid_y: u32 = ui_state.fluid.fire_y;
                let mut amount: f32 = ui_state.fluid.amount;
                let mut amount_x: f32 = ui_state.fluid.amount_x;
                let mut amount_y: f32 = ui_state.fluid.amount_y;

                ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                ui_state
                    .fluid
                    .add_velocity(fluid_x, fluid_y, amount_x, amount_y);
                ui_state.fluid.step();
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
                commands,
                frames,
                query_fluid,
                &mut windows,
                // ui_state,
            );
        });
    });
}
