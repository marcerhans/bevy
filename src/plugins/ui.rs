use super::*;
use bevy::window::PresentMode;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_framepace::FramepaceSettings;
use bevy_inspector_egui::egui::style::Spacing;
use n_body;
// use bevy_inspector_egui::WorldInspectorPlugin;

pub const WIDTH_MARGIN: f32 = 256.0; // Note: If this is smaller than a slider, then it will not be the "true" width.
pub const HEIGHT_MARGIN: f32 = 32.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: APPLICATION_NAME.to_string(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(UiState::default())
        .add_startup_system(initialize)
        .add_system(ui_system);
    }
}

type ProjectGenUiFn<T> = Box<dyn Fn(&mut egui::Ui, &mut T) + Send + Sync>;
enum Project {
    NBodySim(ProjectGenUiFn<n_body::State>),
}

#[derive(Resource)]
struct UiState {
    current_project: Project,
    fps_slider_val: f64,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_project: Project::NBodySim(Box::new(n_body::gen_ui)),
            fps_slider_val: 120.0,
        }
    }
}

fn initialize(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx
        .ctx_mut()
        .set_visuals(egui::Visuals { ..default() });
}

fn ui_system(
    asset_server: ResMut<AssetServer>,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    counter: Res<misc::fps::Counter>,
    mut framepace_settings: ResMut<FramepaceSettings>,
    mut n_body_state: ResMut<n_body::State>,
) {
    let github_image_id =
        egui_ctx.add_image(asset_server.load("images/github/github-mark-white.png"));

    egui::SidePanel::left("side_panel_left")
        .default_width(WIDTH_MARGIN)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |mut ui| {
                    ui.heading("Projects");
                    ui.separator();

                    let mut n_body_simulation_button = egui::Button::new("n-body simulation");
                    if let Project::NBodySim(_) = ui_state.current_project {
                        n_body_simulation_button =
                            n_body_simulation_button.fill(egui::Color32::BLACK);
                    }
                    if ui.add(n_body_simulation_button).clicked() {
                        ui_state.current_project = Project::NBodySim(Box::new(n_body::gen_ui));
                        println!("hej");
                    }

                    ui.heading("Global Controls");
                    ui.separator();

                    if ui.add(egui::Button::new("Reset")).clicked() {
                        *ui_state = UiState::default();
                    }

                    ui.add(
                        egui::Slider::new(&mut ui_state.fps_slider_val, 10.0..=1000.0)
                            .logarithmic(true)
                            .text("FPS limit"),
                    );
                    framepace_settings.limiter =
                        bevy_framepace::Limiter::from_framerate(ui_state.fps_slider_val);

                    ui.heading("Project Controls");
                    ui.separator();

                    match &ui_state.current_project {
                        Project::NBodySim(fun) => fun(&mut ui, &mut n_body_state),
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.hyperlink_to(
                            "Source code / repository",
                            "https://github.com/marcerhans/bevy",
                        );
                        ui.image(github_image_id, bevy_egui::egui::vec2(40.0, 40.0));
                        ui.separator();
                    });
                },
            );
        });

    egui::TopBottomPanel::top("top_panel")
        .default_height(HEIGHT_MARGIN)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let fps = format!("FPS: {:.2}", counter.get());

                ui.style_mut().visuals.override_text_color = match counter.get() as i64 {
                    i64::MIN..=29 => Some(egui::Color32::RED),
                    30..=59 => Some(egui::Color32::YELLOW),
                    60..=119 => Some(egui::Color32::GREEN),
                    120.. => Some(egui::Color32::GOLD),
                };

                Some(egui::Color32::from_rgb(255, 255, 255));

                ui.label(fps);
                ui.separator();
            });
        });
}
