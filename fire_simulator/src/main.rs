use bevy::{asset::SourceMeta, prelude::*, render::texture::PixelInfo};
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component)]
struct Material {
    name_type: String,
    flamability: f32,
    color: Color,
    width: f32,
    height: f32,
    position_x: f32,
    position_y: f32,
}

fn print_sistem(query: Query<&Material>) {
    for material in &query {
        println!("{}", material.name_type);
    }
}

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(start_system)
        .add_startup_system(configure_visuals)
        .add_startup_system(configure_ui_state)
        .add_system(ui)
        // .add_system(print_sistem)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    is_window_open: bool,
}

fn start_system(mut commands: Commands) {
    commands.spawn(
        (Material {
            name_type: String::from("Wood"),
            flamability: 0.23,
            color: Color::rgb(0.4, 0.4, 0.4),
            width: 0.5,
            height: 0.5,
            position_x: 0.5,
            position_y: 0.5,
        }),
    );
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

fn ui(mut egui_ctx: ResMut<EguiContext>) {
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
