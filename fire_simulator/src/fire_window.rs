use bevy_egui::egui::{self, Ui};

use crate::fluid::{FluidMatrix, N};

pub fn fire_window(ui: &mut Ui, mut fire: FluidMatrix) -> Option<FluidMatrix> {
    ui.separator();

    ui.add(egui::Slider::new(&mut fire.delta_time, 0.0..=3.0).text("Timestep"));
    if ui.button("Increment").clicked() {
        fire.delta_time += 0.1;
    }
    ui.add(egui::Slider::new(&mut fire.diffusion, 0.0..=10.0).text("Diffusion"));
    if ui.button("Increment").clicked() {
        fire.diffusion += 0.001;
    }

    ui.add(egui::Slider::new(&mut fire.viscosity, 0.0..=1.0).text("Viscosity"));
    if ui.button("Increment").clicked() {
        fire.viscosity += 0.0000001;
    }

    ui.add(egui::Slider::new(&mut fire.fire_x, 0..=N - 2).text("Fire Y"));
    if ui.button("Increment").clicked() {
        fire.fire_x += 1;
    }
    ui.add(egui::Slider::new(&mut fire.fire_y, 0..=N - 2).text("Fire X"));
    if ui.button("Increment").clicked() {
        fire.fire_y += 1;
    }
    ui.add(egui::Slider::new(&mut fire.amount, 0.0..=255.0).text("Power"));
    if ui.button("Increment").clicked() {
        fire.amount += 1.0;
    }

    ui.add(egui::Slider::new(&mut fire.amount_x, -200.0..=200.0).text("Velocity Y"));
    if ui.button("Increment").clicked() {
        fire.amount_x += 1.0;
    }

    ui.add(egui::Slider::new(&mut fire.amount_y, -200.0..=200.0).text("Velocity X"));
    if ui.button("Increment").clicked() {
        fire.amount_y += 1.0;
    }

    ui.add(egui::Slider::new(&mut fire.frames, 0..=100).text("Frames"));
    if ui.button("Increment").clicked() {
        fire.frames += 1;
    }

    Some(fire)
}
