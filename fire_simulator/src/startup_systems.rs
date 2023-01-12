use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Fire;
use crate::Fluid;
use crate::Material;
use crate::UiState;

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_ui_state(mut ui_state: ResMut<UiState::UiState>) {
    ui_state.is_window_open = true;
    ui_state.material = Material::default();
    ui_state.fire = Fire::default();
    ui_state.fluid = Fluid::FluidMatrix::new(1.0, 0., 0.);
    ui_state.new_material = false;
    ui_state.new_fire = false;
    ui_state.new_fluid = false;
    ui_state.material_window = false;
    ui_state.fire_window = false;
    ui_state.fluid_window = false;
}
