use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
};

use crate::Fluid;
use crate::Materials;

#[derive(Default, Resource)]
pub struct MaterialChangebility {
    pub side_panel_modify: bool,
    pub material_for_change: Materials,
    pub fluid_for_change: Fluid::FluidMatrix,
    pub material_change_flag: bool,
    pub fire_change_flag: bool,
    pub materials_entities_id: Vec<Entity>,
}
