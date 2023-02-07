use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
}
;

#[derive(Default, Resource)]
pub struct Windows{
    pub side_panel_modify:bool,
}