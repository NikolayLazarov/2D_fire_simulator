use bevy::{prelude::*, sprite::collide_aabb::collide};

use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Fire;
use crate::Fluid;
use crate::Fluid::N;
use crate::Material;
use crate::UiState;

pub fn fluid_and_materials(
    mut query: Query<&Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState::UiState>,
) {
}

pub fn fluid_sys(
    mut query_material: Query<& mut Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut ui_state: ResMut<UiState::UiState>,
) {
    // ui_state.fluid.step();
    // ui_state.fluid.render_density();
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Fluid").show(ui.ctx(), |ui| {
            ui.label("Fluid");

            for i in 0..N - 1 {
                for j in 0..N - 1 {
                    let x: u32 = i;
                    let y: u32 = j;
                    let d = ui_state.fluid.get_density()[Fluid::IX(x, y) as usize];
                    // ui.monospace(format!("{}", d));
                    //
                    //use d as alpha color a
                    //no stroke
                    //square(x,y, Scale)


                    for mut material in  query_material.iter_mut(){
                        let collision = collide(
                            Vec3::new(x as f32, y as f32, 1.0),
                            Vec2::new(1.0, 1.0),
                            Vec3::new(material.position_x, material.position_y, 1.0),
                            Vec2::new(material.width, material.height),
                        );
                        if let Some(_) = collision{
                            if material.fuel <= 0.0{
                                
                                continue;
                            }
                             material.fuel = material.fuel - d;

                            ui.label("Material collides with Fire");
                        }
                        // ui.label(format!(
                        //     "Material = {}, width = {}, height = {}, x = {}, y = {}",
                        //     material.name_type,
                        //     material.width,
                        //     material.height,
                        //     material.position_x,
                        //     material.position_y
                        // ));
                    }
                }
            }
            // egui::Rect::from_min_max(pos2(0.0, 1.0), pos2(2.0, 3.0));
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

pub fn fire_fetch_system(
    mut query: Query<&Fire>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
) {
    egui::Area::new("Fires").show(egui_ctx.ctx_mut(), |ui| {
        for mut fire in &mut query {
            let mut button = false;
            ui.label(format!(
                "Fire = {}, range = {}, speed{}",
                fire.name, fire.range, fire.speed
            ));
            button = ui.button(fire.name.to_string()).clicked();
        }
    });
}

fn all_elements_system(
    mut query_fires: Query<&Fire>,
    mut query_materials: Query<&Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Central Area").show(ui.ctx(), |ui| {
            for mut material in &mut query_materials {
                let mut button = false;
                ui.label(format!(
                    "Material = {}, width = {}, height = {}, x = {}, y = {}",
                    material.name_type,
                    material.width,
                    material.height,
                    material.position_x,
                    material.position_y
                ));
                button = ui.button(material.name_type.to_string()).clicked();
            }

            for mut fire in &mut query_fires {
                let mut button = false;
                ui.label(format!(
                    "Fire = {}, range = {}, speed{},  width = {}, height = {}, x = {}, y = {}",
                    fire.name,
                    fire.range,
                    fire.speed,
                    fire.width,
                    fire.height,
                    fire.position_x,
                    fire.position_y
                ));
                button = ui.button(fire.name.to_string()).clicked();
            }
        });
    });
}

pub fn collision_system(
    mut query_fires: Query<&Fire>,
    mut query_materials: Query<&Material>,
    mut egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");

        egui::Area::new("Central Area").show(ui.ctx(), |ui| {
            for fire in query_fires.iter() {
                ui.label(format!(
                    "Fire = {}, range = {}, speed{},  width = {}, height = {}, x = {}, y = {}",
                    fire.name,
                    fire.range,
                    fire.speed,
                    fire.width,
                    fire.height,
                    fire.position_x,
                    fire.position_y
                ));

                //                ui.label(format!("{}",fire.name));
                for material in query_materials.iter() {
                    //vec3 -> x,y,z
                    let collision = collide(
                        Vec3::new(fire.position_x, fire.position_y, 1.0),
                        Vec2::new(fire.width, fire.height),
                        Vec3::new(material.position_x, material.position_y, 1.0),
                        Vec2::new(material.width, material.height),
                    );

                    if let Some(_) = collision {
                        ui.label("Material collides with fire");
                    }
                    ui.label(format!(
                        "Material = {}, width = {}, height = {}, x = {}, y = {}",
                        material.name_type,
                        material.width,
                        material.height,
                        material.position_x,
                        material.position_y
                    ));
                }
            }
        });
    });
}
