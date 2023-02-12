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
use crate::Materials;
use crate::UiState::{self, ui_state};
use crate::Windows;

// fn functionCheckCollision(  list_materials:  <>){
// for mut material in query_material.iter_mut() {
//     let collision = collide(
//         Vec3::new(x as f32, y as f32, 1.0),
//         Vec2::new(1.0, 1.0),
//         Vec3::new(material.position_x, material.position_y, 1.0),
//         Vec2::new(material.width, material.height),
//     );
//     if let Some(_) = collision {
//         if material.fuel <= 0.0 {
//             continue;
//         }
//         material.fuel = material.fuel - d;

//         ui.label("Material collides with Fire");
//     }
// }
// }

fn create_rect(
    ui: &mut Ui,
    r: u8,
    g: u8,
    b: u8,
    windows: &mut ResMut<Windows::Windows>,
    object_flag: bool,
) -> bool {
    let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::click());
    ui.painter().rect(
        rect,
        0.0,
        // egui::Color32::BLUE,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );
    // let mut clicked_rect = false;
    if Response.clicked() && object_flag {
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
    windows: &mut ResMut<Windows::Windows>,
) {
    //remove comments latter
    // for material in query_materials.iter(){
    //     println!("material = {:?}", material);
    //     // commands.entity(material).despawn();
    // }
    // // ui.add( );

    // ui.horizontal(|ui|{

    //     egui::Frame::canvas(ui.style()  )
    // .fill(egui::Color32::RED)
    // .show(ui, |ui| {

    //     // ui.add(

    //     // );

    //     let (response, painter) = ui.allocate_painter(ui .available_size_before_wrap(), egui::Sense::hover());
    //     // let (rect) = ui.allocate ( egui::Rect{ min: Pos2 { x: 2., y: 2. }, max: Pos2 { x: 4., y: 4. }} , egui::Sense::hover());
    //     let mut rect = response.rect;
    //     // rect.min = Pos2 { x: 2., y: 2. };
    //     // rect.max = Pos2 { x: 4., y: 4. };
    //     // height() = ;
    //     painter.add( egui::Shape::Rect(RectShape { rect: egui::Rect{ min: Pos2 { x: 2., y: 2. }, max: Pos2 { x: 4., y: 4. } } , rounding: Rounding::none(), fill: egui::Color32::BLUE, stroke: egui::Stroke::new(3.5,egui::Color32::RED)} ) );
    //     // painter.add(egui::Shape::Rect(RectShape { rect: rect,  rounding: Rounding::none(), fill: egui::Color32::BLUE, stroke: egui::Stroke::new(3.5,egui::Color32::RED) }) );

    // });

    // });
    // i -> y Axis
    // j -> x Axis

    for i in 0..N - 1 {
        ui.horizontal_top(|ui| {
            for j in 0..N - 1 {
                let x: u32 = i;
                let y: u32 = j;
                let mut d = density[Fluid::IX(x, y) as usize];

                let mut material_flag: bool = false;
                let mut fluid_flag: bool = false;

                for mut material in query_materials.iter_mut() {
                    if check_if_material_at_position(x, y, material.position_x, material.position_y)
                    {
                        if material.fuel > 0. && frames > 0 {
                            for fluid in fluids.iter() {
                                let material_in_fire_size = collide(
                                    Vec3 {
                                        x: fluid.fluid_x as f32,
                                        y: fluid.fluid_y as f32,
                                        z: 0.,
                                    },
                                    Vec2 {
                                        x: fluid.fire_size as f32,
                                        y: fluid.fire_size as f32,
                                    },
                                    Vec3 {
                                        x: material.position_x as f32,
                                        y: material.position_y as f32,
                                        z: 0.,
                                    },
                                    //put the sizes parameters when you implement them
                                    Vec2 { x: 1., y: 1. },
                                );
                                if let Some(_) = material_in_fire_size {
                                    material.fuel -= fluid.amount;
                                }
                            }
                        }
                        // if material.fuel > 0. && frames > 0 {
                        //     let [mut up, mut down, mut left, mut right] = [0.; 4];
                        //     if y != 0 {
                        //         up = density[Fluid::IX(i, j - 1) as usize];
                        //     }
                        //     if x != 0 {
                        //         left = density[Fluid::IX(i - 1, j) as usize];
                        //     }
                        //     if x != N - 1 {
                        //         right = density[Fluid::IX(i + 1, j) as usize];
                        //     }
                        //     if j != N - 1 {
                        //         down = density[Fluid::IX(i, j + 1) as usize];
                        //     }
                        //     material.fuel -= up + down + left + right;
                        // }

                        // println!("material fuel = {} ", material.fuel);
                        if material.fuel <= 0. {
                            // let mut entity_ids =
                            let mut cords_flag = false;
                            for mut fluid in fluids.iter_mut() {
                                for cords in fluid.materials_cords.iter() {
                                    if cords.0 == x && cords.1 == y {
                                        cords_flag = true;
                                    }
                                }
                                if cords_flag {
                                    fluid.materials_cords.retain(|&f| f == (x, y));
                                }
                            }
                            //remove from kords --ok
                            //despawn
                            //
                            //false because it changes during the simulation
                            // create_rect(ui, (255 - d as u8), 0, 0, windows, false);
                        } else {
                            let coeficient = material.fuel / 10.;

                            if create_rect(ui, 0, coeficient as u8, 255, windows, true) {
                                windows.material_for_change = material.clone();
                                windows.material_change_flag = true;
                                // commands.entity(material).despawn();
                            }
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
                            x: fluid.fluid_x as f32,
                            y: fluid.fluid_y as f32,
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
                                if x == fluid.fluid_x && y == fluid.fluid_y {
                                    //central pixel of the fire
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
                                        fluid.counter_range = 1;
                                    }
                                    // continue;
                                }
                                //fix check if in range
                                else if fluid.fire_size > 1
                                    && check_if_in_range(
                                        x as i32,
                                        y as i32,
                                        fluid.fluid_x as i32,
                                        fluid.fluid_y as i32,
                                        fluid.counter_range as i32,
                                    )
                                {
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
                                // else if fluid.fire_size == 5 && !check_if_in_range(x as i32, y as i32, fluid.fluid_x as i32, fluid.fluid_y as i32, fluid.counter_range as i32){
                                //     println!("range 5");
                                //     if create_rect(
                                //         ui,
                                //         0 ,
                                //        //maybe here use only the amount -> see how much green makes orange or yellow
                                //         0,
                                //         255,
                                //         windows,
                                //         true,
                                //     ) {
                                //         windows.fluid_for_change = fluid.clone();
                                //         windows.material_change_flag = true;
                                //     }
                                // }
                                else {
                                    if create_rect(
                                        ui, 0,
                                        //maybe here use only the amount -> see how much green makes orange or yellow
                                        0, 255, windows, true,
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

                if d > 255.0 || d <= 0. {
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
                            egui::Color32::from_gray(255 - (d * (frames as f32)) as u8),
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
    mut windows: ResMut<Windows::Windows>,
) {
    //let ten_millis = time::Duration::from_millis(1000 / 24);
    let ten_millis = time::Duration::from_millis(100);

    let now = time::Instant::now();
    let mut frames = 0;

    if ui_state.start_simulation {
        frames = ui_state.fluid.frames;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Fluid").show(ui.ctx(), |ui| {
            ui.label("Fluid");

            if frames > 0 {
                ui_state.new_fluid = false;
                // ui_state.start_simulation = true;
                let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                let mut amount: f32 = ui_state.fluid.amount;
                let mut amount_x: f32 = ui_state.fluid.amount_x;
                let mut amount_y: f32 = ui_state.fluid.amount_y;

                ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
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
