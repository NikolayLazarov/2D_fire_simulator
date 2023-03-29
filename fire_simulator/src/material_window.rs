use bevy_egui::egui::{self, Ui};

use crate::{fluid::N, Materials};

pub fn window(
    ui: &mut Ui,
    /*mut name_material: String, mut position_x: u32, mut position_y: u32, mut flammability: u32,*/
    mut material: Materials,
) -> Option<Materials> {
    // let mut change_material_button = false;
    ui.heading("here we go");
    ui.heading("Material");

    ui.horizontal(|ui| {
        ui.label("Your material: ");
        ui.text_edit_singleline(&mut material.name_material);
    });

    ui.add(egui::Slider::new(&mut material.position_y, 0..=N - 1).text("X axys"));
    if ui.button("Increment").clicked() {
        material.position_y += 1;
    }

    ui.add(egui::Slider::new(&mut material.position_x, 0..=N - 1).text("Y axys"));
    if ui.button("Increment").clicked() {
        material.position_x += 1;
    }

    ui.add(egui::Slider::new(&mut material.flammability, 0..=100).text("Flammability"));
    if ui.button("Increment").clicked() {
        material.flammability += 1;
    }

    return Some(material);
}
