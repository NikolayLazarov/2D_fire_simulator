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

fn create_rect(ui: &mut Ui, r: u8, g: u8, b: u8, windows: &mut ResMut<Windows::Windows>) {
    let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::click());
    ui.painter().rect(
        rect,
        0.0,
        // egui::Color32::BLUE,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );
    // let mut clicked_rect = false;
    if Response.clicked() {
        windows.side_panel_modify = true;
    }

    // if clicked_rect {
    //     windows.side_panel_modify = true;
    // }
    // println!("Responce = {:?}", Response)
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

fn render_density(
    ui: &mut Ui,
    density: &Vec<f32>,
    mut query_materials: Query<&mut Materials>,
    mut commands: Commands,
    frames: u32,
    mut fluids: Query<&mut FluidMatrix>,
    windows: &mut ResMut<Windows::Windows>, // ui_state: ResMut<UiState::UiState>,
) {
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

    for i in 0..N - 1 {
        ui.horizontal_top(|ui| {
            for j in 0..N - 1 {
                // if frames == 0{
                //     create_rect(ui, 0 , 0, 0);
                //     continue;
                // }

                let x: u32 = i;
                let y: u32 = j;
                let mut d = density[Fluid::IX(x, y) as usize];

                let mut material_flag: bool = false;
                let mut fluid_flag: bool = false;
                for mut material in query_materials.iter_mut() {
                    if check_if_material_at_position(x, y, material.position_x, material.position_y)
                    {
                        material.fuel -= d;
                        if material.fuel <= 0. {
                            create_rect(ui, (255 - d as u8), 0, 0, windows);
                        } else {
                            // let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                            // let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                            // let mut amount: f32 = ui_state.fluid.amount;
                            // let mut amount_x: f32 = ui_state.fluid.amount_x;
                            // let mut amount_y: f32 = ui_state.fluid.amount_y;
                            // ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                            // ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
                            create_rect(ui, 0, 0, 255, windows);
                        }
                        material_flag = true;
                    }
                }
                if material_flag == true {
                    continue;
                }

                for fluid in fluids.iter() {
                    // println!("count =  {}",count);
                    // // println!("loop");
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
                            x: fluid.fire_range as f32,
                            y: fluid.fire_range as f32,
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
                            if d > 255. {
                                d = 255.;
                            }
                            // println!("{} ", d);
                            // fluid_false == false
                            //put the renge to be in red
                            // the center in yellow
                            //counter
                            //if count =1  -> yellow
                            // else if  count -> red
                            if fluid.fire_range % 2 == 1 {
                                if fluid.counter_range == fluid.fire_range / 2 {
                                    create_rect(ui, 255 - d as u8, 255 - d as u8, 0, windows);
                                } else {
                                    create_rect(ui, 255 - d as u8, d as u8, 0, windows);
                                }
                            } else if fluid.fire_range % 2 == 0 {
                                create_rect(ui, 255 - d as u8, d as u8, 0, windows);
                                // else if fluid.counter_range == fluid.fire_range{

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
                    create_rect(ui, d as u8, 0, 0, windows);
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
                        egui::Stroke::new(9.0, egui::Color32::from_gray(255 - d as u8)), //from_rgb(r, g, b)),
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
    let ten_millis = time::Duration::from_millis(1000 / 24);
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
                //prints density
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                ui_state.fluid.frames -= 1;
                if ui_state.fluid.frames == 0 {
                    ui_state.start_simulation = false;
                }
            }
            // println!("Entity_ fluid = {:?}", query_fluid);
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

            // if windows.side_panel_modify == true{
            //     println!("it was here");

            //     egui::Window::new("Window")
            // .vscroll(true)
            // .open(&mut ui_state.window_change_materials)
            // .show(ui.ctx(), |ui| {
            //     println!("Now is here");
            //     ui.label("Windows can be moved by dragging them.");
            //     ui.label("They are automatically sized based on contents.");
            //     ui.label("You can turn on resizing and scrolling if you like.");
            //     ui.label("You would normally chose either panels OR windows.");
            // });

            //  }
        });

        // egui::Window::new("Window")
        // .open(&mut ui_state.window_change_materials)
        // .show(egui_ctx.ctx_mut(), |ui|{

        // })
    });
}

pub fn material_fetch_system(
    mut query: Query<&Materials>,
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
