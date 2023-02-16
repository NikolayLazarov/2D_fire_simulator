use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Fluid;
use crate::Materials;
use crate::UiState;
use crate::MaterialChangability;

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_ui_state(mut ui_state: ResMut<UiState::UiState>) {
    ui_state.is_window_open = true;
    ui_state.material = Materials::default();
    ui_state.fluid = Fluid::FluidMatrix::new(); //(0.1, 0., 0.0000001);
    ui_state.new_material = false;
    ui_state.new_fire = false;
    ui_state.new_fluid = false;
    ui_state.start_simulation = false;
    ui_state.material_window = false;
    ui_state.fire_window = false;
    ui_state.fluid_window = false;
    ui_state.window_change_materials = false;
}

pub fn configure_windows(mut windows: ResMut<MaterialChangability::MaterialChangebility>) {
    windows.side_panel_modify = false;
    windows.fluid_for_change = Fluid::FluidMatrix::new();// (1., 0., 0.);
    windows.material_for_change = Materials::default();
    windows.materials_entities_id = vec![];
}
