use bevy::ui;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use std::{thread, time};

use bevy_egui::egui::{vec2, Ui};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// use crate::Fire;
use crate::Fluid::N;
use crate::Fluid::{self, FluidMatrix};
use crate::Material;
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

fn render_density(ui: &mut Ui, density: &Vec<f32>
    // ,    mut query_material: Query<&mut dyn Segment>
) {

    for i in 0..N - 1 {
        ui. horizontal_top (|ui| {
            for j in 0..N - 1 {
                // for mut fluid in query_fluid.iter_mut(){
                //         fluid.get_density();
                // }
                //print squares
                let x: u32 = i;
                let y: u32 = j;
                let d = density[Fluid::IX(x, y) as usize];
                let (rect, Response) = ui.allocate_at_least(vec2(0.5, 3.0), egui::Sense::hover());
                ui.painter().
                 rect(
                    rect,
                    0.0,
                    egui::Color32:: from_rgb(d as u8, 50, 50) ,//:from_rgb(50, 100, 150).linear_multiply(0.25), //:from_gray(d as u8),
                    egui::Stroke::new(9.0, egui::Color32:: from_rgb(d as u8, 0, 0)), /* :none()*/
                );


                // functionCheckCollision(query_material);
            }
        } ) ;
    }
    // println!(" It is here");
}

//function for constraining values
// fn constrain( mut value: f32, min: f32, max: f32) -> &'static mut f32 {
//     if value < min {
//         value = min;
//     }
//     if value > max {
//         value = max;
//     }
//    &mut value
// }

// function for faiding away
// pub fn fade_density(
//      density: & Vec<f32> ) {

//PROBLEM HERE
//     for mut cell in density.iter_mut(){
//         let mut old = cell.clone();
//         cell = &mut old - 0.1;
//         cell = constrain(cell - (0.1 as f32),0.255.);    
//     }
// }

pub fn fluid_sys(
    // mut query_material: Query<&mut Material>,
    mut query_fluid: Query<&mut FluidMatrix>,
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
                // if ui_state.new_fluid{

                let mut fluid_x: u32 = ui_state.fluid.fluid_x;
                let mut fluid_y: u32 = ui_state.fluid.fluid_y;
                let mut amount: f32 = ui_state.fluid.amount;
                let mut amount_x: f32 = ui_state.fluid.amount_x;
                let mut amount_y: f32 = ui_state.fluid.amount_y;

                println!("frame = {}", frames);
                // for i in 1.. 20{
                ui_state.fluid.add_density(fluid_x, fluid_y, amount);
                ui_state.fluid.add_velocity(fluid_x, fluid_y, 200.0, 200.0);
                ui_state.fluid.step();

                // print_squares(ui,ui_state.fluid.get_density() );
                // println!("sec {}",i);
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                // }
                ui_state.fluid.frames -= 1;
                if ui_state.fluid.frames == 0 {
                    ui_state.new_fluid = false;
                }
            }
            // else{
            // ui_state.fluid.step();
            render_density(ui, ui_state.fluid.get_density());
            // todo!();
            // fade_density( ui_state.fluid.get_density());

            // }
        });
    });
}

pub fn material_fetch_system(
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
