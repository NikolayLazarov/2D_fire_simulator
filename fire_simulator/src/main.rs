use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

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
        .add_system(ui_state)
        // .add_system(print_sistem)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    is_window_open: bool,
    label: String,
    materials: Vec<Material>,
    button: bool,
    button2: bool,
    vector: Vec<u32>,
}

impl UiState {
    fn all_materials(&mut self, ui: &mut egui::Ui) {
        for element in &self.materials {
            ui.label(&element.name_type);
        }
    }
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
    ui_state.button = false;
    ui_state.button2 = false;
    ui_state.vector = vec![1, 2, 3];
}

fn ui_state(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
) {
    let mut but1 = false;
    let mut but2 = false;
    let mut but3 = false;

    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Wood");
                but1 = ui.button("Wood").clicked();
            });

            ui.horizontal(|ui| {
                ui.label("Metall");
                but2 = ui.button("Invert").clicked();
            });

            ui.horizontal(|ui| {
                ui.label("Other");
                but3 = ui.button("Other").clicked();
            });

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);

                //new material parameters
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
            for i in &ui_state.vector {
                // ui_state.material.ui_content(ui,*i);
                ui.label(i.to_string());
            }
        });

        ui.separator();

        ui.label("Scene2");

        egui::Area::new("Central Area2").show(ui.ctx(), |ui| {
            ui_state.all_materials(ui);
        });

        if but1 {
            ui_state.button = !ui_state.button;
        }

        if ui_state.button {
            ui_state.vector.push(4);
            ui_state.button = false;
        }
    });

    if but2 || but3 {
        println!("button is clicked");
        println!("Label is {}", ui_state.label);

        ui_state.button2 = !ui_state.button2;
    }

    if ui_state.button2 {
        ui_state.materials.push(Material {
            name_type: String::from("Wood"),
            flamability: 0.23,
            color: Color::rgb(0.4, 0.4, 0.4),
            width: 0.5,
            height: 0.5,
            position_x: 0.5,
            position_y: 0.5,
        });

        ui_state.button2 = false;
    }
}

#[derive(Component, Debug)]
struct Material {
    name_type: String,
    flamability: f32,
    color: Color,
    width: f32,
    height: f32,
    position_x: f32,
    position_y: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name_type: String::from("Material"),
            flamability: 0.5,
            color: Color::BEIGE,
            width: 4.5,
            height: 4.5,
            position_x: 4.5,
            position_y: 4.5,
        }
    }
}

impl Material {
    fn ui_content(&mut self, ui: &mut egui::Ui) {
        ui.label("This is text");
    }
}
#[derive(Component)]
struct Materials {
    materials: Vec<Material>,
}

impl Default for Materials {
    fn default() -> Self {
        Self {
            materials: Vec::new(),
        }
    }
}
