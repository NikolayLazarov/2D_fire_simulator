use bevy::prelude::*;

use crate::fluid;
use crate::Materials;

#[derive(Default, Resource)]
pub struct ElementChangebilityContext {
    pub side_panel_modify: bool,
    pub material_for_change: Materials,
    pub fluid_for_change: fluid::FluidMatrix,
    pub material_change_flag: bool,
    pub fire_change_flag: bool,
    pub materials_entities_id: Vec<Entity>,
}
