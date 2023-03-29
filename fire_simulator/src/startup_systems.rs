use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::element_changability;
use crate::fluid;
use crate::ui_state;
use crate::Materials;

pub fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn configure_ui_state(mut ui_state: ResMut<ui_state::UiState>) {
    ui_state.is_window_open = true;
    ui_state.material = Materials::default();
    ui_state.fluid = fluid::FluidMatrix::new();
    ui_state.new_material = false;
    ui_state.new_fluid = false;
    ui_state.start_simulation = false;
    ui_state.material_window = false;
    ui_state.fire_window = false;
    ui_state.fluid_window = false;
    ui_state.created_fire = false;
    ui_state.window_change_materials = false;
    ui_state.restart_simulation = false;
    ui_state.counter_fire_size = 1;
}

pub fn configure_windows(mut windows: ResMut<element_changability::ElementChangebilityContext>) {
    windows.side_panel_modify = false;
    windows.fluid_for_change = fluid::FluidMatrix::new();
    windows.material_for_change = Materials::default();
    windows.materials_entities_id = vec![];
}
