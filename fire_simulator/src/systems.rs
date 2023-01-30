use bevy::ui;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_egui::egui::epaint::RectShape;
use std::{thread, time};

use bevy_egui::egui::{vec2, Pos2, Rounding, Ui};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Fluid::N;
use crate::Fluid::{self, FluidMatrix};
use crate::Materials;
use crate::UiState;

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

fn create_rect(ui: &mut Ui, r: u8, g: u8, b: u8) {
    let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
    ui.painter().rect(
        rect,
        0.0,
        // egui::Color32::BLUE,
        egui::Color32::from_rgb(r, g, b),
        egui::Stroke::new(9.0, egui::Color32::from_rgb(r, g, b)),
    );
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
    // ui_state: ResMut<UiState::UiState>,
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
                let x: u32 = i;
                let y: u32 = j;
                let d = density[Fluid::IX(x, y) as usize];

                let mut material_flag: bool = false;
                for mut material in query_materials.iter_mut() {
                    if check_if_material_at_position(x, y, material.position_x, material.position_y)
                    {
                        material.fuel -= d;
                        if material.fuel <= 0. {
                            create_rect(ui, d as u8, 0, 0);
                        } else {
                            // let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                            // let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                            // let mut amount: f32 = ui_state.fluid.amount;
                            // let mut amount_x: f32 = ui_state.fluid.amount_x;
                            // let mut amount_y: f32 = ui_state.fluid.amount_y;
                            // ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                            // ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
                            create_rect(ui, 0, 0, 255);
                        }
                        material_flag = true;
                    }
                }
                if material_flag == true {
                    continue;
                }

                create_rect(ui, d as u8, 0, 0);
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
) {
    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();
    let mut frames = 0;
    if ui_state.new_fluid {
        frames = ui_state.fluid.frames;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Fluid").show(ui.ctx(), |ui| {
            ui.label("Fluid");

            if frames > 0 {
                let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                let mut amount: f32 = ui_state.fluid.amount;
                let mut amount_x: f32 = ui_state.fluid.amount_x;
                let mut amount_y: f32 = ui_state.fluid.amount_y;

                ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
                ui_state.fluid.step();
                //prints density
                let mut vector: Vec<(u32, u32, f32)> = vec![];
                for i in 0..N - 1 {
                    for j in 0..N - 1 {
                        let x: u32 = i;
                        let y: u32 = j;
                        let d = ui_state.fluid.get_density()[Fluid::IX(x, y) as usize];
                        // print!("{} ", d);
                        if d > 1.0 {
                            vector.push((x, y, d));
                        }
                    }
                    println!();
                }
                println!("Out of bound: {:?}", vector);

                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                ui_state.fluid.frames -= 1;
                if ui_state.fluid.frames == 0 {
                    ui_state.new_fluid = false;
                }
            }
            render_density(
                ui,
                ui_state.fluid.get_density(),
                query_materials,
                commands,
                // ui_state,
            );
        });
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
