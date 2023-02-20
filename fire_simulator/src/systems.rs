use bevy::ecs::query;
use bevy::render::view::window;
use bevy::ui;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_egui::egui::epaint::RectShape;
use std::{thread, time};

use bevy_egui::egui::{vec2, Pos2, Rounding, Ui};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Fluid::N;
use crate::Fluid::{self, FluidMatrix};
use crate::MaterialChangability;
use crate::Materials;
use crate::UiState::{self, ui_state};

fn create_rect(
    ui: &mut Ui,
    r: u8,
    g: u8,
    b: u8,
    windows: &mut ResMut<MaterialChangability::MaterialChangebility>,
    object_flag: bool,
) -> bool {
    let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::click());
    ui.painter().rect(
        rect,
        0.0,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );    if Response.clicked() && object_flag {
        windows.side_panel_modify = true;
        true
    } else {
        false
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
    range: i32,
) -> bool {
    let down_range = -range;
    let mut flag_x = false;
    let mut flag_y = false;

    if x_material - x_cord <= range && x_material - x_cord >= down_range {
        flag_x = true;
    }
    if y_material - y_cord <= range && y_material - y_cord >= down_range {
        flag_y = true;
    }

    flag_y && flag_x
}
fn render_density(
    ui: &mut Ui,
    density: &Vec<f32>,
    mut query_materials: Query<&mut Materials>,
    mut commands: Commands,
    frames: u32,
    mut fluids: Query<&mut FluidMatrix>,
    windows: &mut ResMut<MaterialChangability::MaterialChangebility>,
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
                                //flag to check if material is in the range of the fire
                                let material_in_fire_range = collide(
                                    Vec3 {
                                        x: fluid.fire_x as f32,
                                        y: fluid.fire_y as f32,
                                        z: 0.,
                                    },
                                    Vec2 {
                                        x: (fluid.fire_size + fluid.fire_range) as f32,
                                        y: (fluid.fire_size + fluid.fire_range) as f32,
                                    },
                                    Vec3 {
                                        x: material.position_x as f32,
                                        y: material.position_y as f32,
                                        z: 0.,
                                    },
                                    //put the sizes parameters when you implement them
                                    Vec2 { x: 1., y: 1. },
                                );
                                //if the material is in range -> burn it
                                if let Some(_) = material_in_fire_range {
                                    material.fuel -= fluid.amount * (material.flammability as f32 /100 as f32 );
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
                        }

                        //if the material has been burnt
                        if material.fuel <= 0. {
                            // let mut entity_ids ={}
                            let mut cords_flag = false;
                            for mut fluid in fluids.iter_mut() {
                                //check if material is still in the material list
                                for cords in fluid.materials_cords.iter() {
                                    if cords.0 == x && cords.1 == y {
                                        cords_flag = true;
                                    }
                                }
                                //removes the material from the cords list
                                if cords_flag {
                                    //here should be a new black rect or one that is goig to be filled with something
                                    fluid.materials_cords.retain(|&f| f == (x, y));
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
                    let collision = collide(
                        Vec3 {
                            x: fluid.fire_x as f32,
                            y: fluid.fire_y as f32,
                            z: 0.,
                        },
                        Vec2 {
                            x: fluid.fire_size as f32,
                            y: fluid.fire_size as f32,
                        },
                        Vec3 {
                            x: x as f32,
                            y: y as f32,
                            z: 0.,
                        },
                        Vec2 { x: 1., y: 1. },
                    );

                    if fluid_flag == false {
                        if let Some(_) = collision {
                            if fluid.fire_size % 2 == 1 {
                                // println!("size = {}", fluid.fire_size);
                                if x == fluid.fire_x && y == fluid.fire_y {
                                    //central pixel of the fire
                                    // println!("center");
                                    if create_rect(
                                        ui,
                                        255 as u8,
                                        255 - (fluid.amount) as u8,
                                        fluid.amount as u8,
                                        windows,
                                        true,
                                    ) {
                                        windows.fluid_for_change = fluid.clone();
                                        windows.material_change_flag = true;
                                        // fluid.counter_range = 1;
                                    }
                                    // continue;
                                }
                                //fix check if in range
                                else if fluid.fire_size > 1
                                    && check_if_in_range(
                                        x as i32,
                                        y as i32,
                                        fluid.fire_x as i32,
                                        fluid.fire_y as i32,
                                        fluid.counter_range as i32,
                                    )
                                {
                                    // println!("range > 1");
                                    if create_rect(
                                        ui,
                                        255 as u8,
                                        fluid.amount as u8,
                                        0,
                                        windows,
                                        true,
                                    ) {
                                        windows.fluid_for_change = fluid.clone();
                                        windows.material_change_flag = true;
                                        // fluid.counter_range = 2;
                                    }
                                }
                                //fix it
                                else if fluid.fire_size == 5
                                    && !check_if_in_range(
                                        x as i32,
                                        y as i32,
                                        fluid.fire_x as i32,
                                        fluid.fire_y as i32,
                                        fluid.counter_range as i32,
                                    )
                                {
                                    if create_rect(
                                        ui, 255,
                                        //maybe here use only the amount -> see how much green makes orange or yellow
                                        0, 0, windows, true,
                                    ) {
                                        windows.fluid_for_change = fluid.clone();
                                        windows.material_change_flag = true;
                                    }
                                } else {
                                    // println!("range else");
                                    if create_rect(
                                        ui, 255,
                                        //maybe here use only the amount -> see how much green makes orange or yellow
                                        0, 0, windows, true,
                                    ) {
                                        windows.fluid_for_change = fluid.clone();
                                        windows.material_change_flag = true;
                                    }
                                }
                            } else if fluid.fire_size % 2 == 0 {
                                //let blue = 255 - 50;

                                if create_rect(ui, 255, 255, 0, windows, true) {
                                    // if create_rect(ui, 255 - d as u8, d as u8, 0, windows, true) {
                                    windows.fluid_for_change = fluid.clone();
                                    windows.fire_change_flag = true;
                                }
                                // else if fluid.counter_range == fluid.fire_size{

                                // }
                            }

                            fluid_flag = true;
                            //see if this stops others from emiting
                            continue;
                        }
                    }
                    // commands.entity(fluid).despawn();
                }
                if fluid_flag == true {
                    continue;
                }

                if d > 255.0 || d < 0. {
                    //black for where there is not density or is over
                    d = 0.0;
                    create_rect(ui, d as u8, 0, 0, windows, false);
                } else {
                    //colored depending on the intensity of the d
                    // create_rect(ui, (255 - d as u8), d as u8, 0);

                    //works at the end

                    let (rect, Response) =
                        ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
                    ui.painter().rect(
                        rect,
                        0.0,
                        // egui::Color32::BLUE,
                        egui::Color32::from_gray(d as u8),
                        egui::Stroke::new(
                            9.0,
                            // egui::Color32::from_gray((d * (frames as f32)) as u8),
                            egui::Color32::from_gray((d * 100.0) as u8),
                        ), //from_rgb(r, g, b)),
                    );
                    // create_rect(ui, (255 - d as u8), d as u8, 0);
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
    mut windows: ResMut<MaterialChangability::MaterialChangebility>,
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
