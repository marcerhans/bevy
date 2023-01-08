use super::*;
use bevy_egui::egui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {}
}

fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {}

pub fn gen_ui(ui: &mut bevy_egui::egui::Ui, state: &mut state::State) {
    if ui.add(egui::Button::new("Reset")).clicked() {
        *state = state::State::default();
    }

    ui.add(
        egui::Slider::new(&mut state.body_amount, 2..=MAX_BODIES)
            .logarithmic(true)
            .text("#Bodies"),
    );

    ui.add(
        egui::Slider::new(&mut state.line_distance_limit, 50.0..=1000.0)
            .logarithmic(true)
            .text("Line dist. limit"),
    );

    ui.horizontal(|ui| {
        let mut colors = &mut state.line_color.into();
        ui.label("Line color:");
        ui.color_edit_button_rgba_unmultiplied(&mut colors);
        state
            .line_color
            .set_r(colors[0])
            .set_g(colors[1])
            .set_b(colors[2])
            .set_a(colors[3]);
    });

    ui.horizontal(|ui| {
        let mut colors = &mut state.body_color.into();
        ui.label("Body color:");
        ui.color_edit_button_rgba_unmultiplied(&mut colors);
        state
            .body_color
            .set_r(colors[0])
            .set_g(colors[1])
            .set_b(colors[2])
            .set_a(colors[3]);
    });

    ui.horizontal(|ui| {
        ui.label("Draw Lines:");
        ui.add(egui::Checkbox::new(&mut state.line_draw, ""));
    });
}
