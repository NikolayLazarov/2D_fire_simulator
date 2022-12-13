use bevy::{prelude::*, render::view::window};
use bevy_egui::{egui::{self, Stroke, Ui}, EguiPlugin, EguiContext};

fn main() {
    App::new()
    .init_resource::<UiState>()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_startup_system(configure_visuals)
    .add_startup_system(configure_ui_state)
    .add_system(ui)
    .run();



}

#[derive(Default, Resource)]
struct UiState {
    is_window_open: bool,
}


fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

fn ui(
    mut egui_ctx: ResMut<EguiContext>,

    mut ui_state: ResMut<UiState>,
){

    egui::SidePanel::right("side_panel")
    .default_width(200.0)
    .resizable(true)
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Side Panel");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.allocate_space(ui.available_size());
        });
    });


    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Top pannel");
     });

     egui::TopBottomPanel::bottom("bottom_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Bottom pannel");
     });


    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Scene");
    });    
}
