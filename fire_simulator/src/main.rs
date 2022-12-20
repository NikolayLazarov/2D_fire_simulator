use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_visuals)
        .add_startup_system(configure_ui_state)
        .add_system(ui_state)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    is_window_open: bool,
    // materials: Vec<Material>,
    new_material: bool,
    material: Material,
}

// impl UiState {
//     fn all_materials(&mut self, ui: &mut egui::Ui,) {
//         // for element in &self.materials {
//         //     ui.label(&element.name_type);
//         //     ui.label(&element.width.to_string());
//         //     ui.separator();
//         // }
//     }
// }

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
    ui_state.material = Material::default();
    ui_state.new_material = false;
}

fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
) {
    let mut new_button = false;

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.heading("Material");

            ui.horizontal(|ui| {
                ui.label("Your material: ");
                // ui.text_edit_singleline( &mut ui_state.name);
                ui.text_edit_singleline(&mut ui_state.material.name_type);
            });

            ui.add(egui::Slider::new(&mut ui_state.material.width, 0..=30).text("Width"));
            if ui.button("Increment").clicked() {
                ui_state.material.width += 1;
            }

            ui.add(egui::Slider::new(&mut ui_state.material.height, 0..=30).text("Height"));
            if ui.button("Increment").clicked() {
                ui_state.material.height += 1;
            }

            ui.add(egui::Slider::new(&mut ui_state.material.position_x, 0..=30).text("X axys"));
            if ui.button("Increment").clicked() {
                ui_state.material.position_x += 1;
            }

            ui.add(egui::Slider::new(&mut ui_state.material.position_y, 0..=30).text("Y axys"));
            if ui.button("Increment").clicked() {
                ui_state.material.position_y += 1;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("New");
                new_button = ui.button("New").clicked();
            });

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

        egui::Area::new("Central Area").show(ui.ctx(), |ui| {
            // ui_state.all_materials(ui);
        });
    });

    if new_button {
        ui_state.new_material = !ui_state.new_material;
    }

    if ui_state.new_material {
        commands.spawn(ui_state.material.clone());
        ui_state.new_material = false;
    }
}

#[derive(Component, Debug, Clone)]
struct Material {
    name_type: String,
    flamability: f32,
    color: Color,
    width: u32,
    height: u32,
    position_x: u32,
    position_y: u32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flamability: 0.5,
            color: Color::BEIGE,
            width: 4,
            height: 5,
            position_x: 5,
            position_y: 5,
        }
    }
}

impl Material {
    fn ui_content(&mut self, ui: &mut egui::Ui) {
        ui.label("This is text");
    }
}
