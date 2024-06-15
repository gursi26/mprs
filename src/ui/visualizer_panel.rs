use crate::state::state::AppState;
use crate::ui::toggle_button::toggle;
use eframe::egui::{self, Ui};

pub fn draw_visualizer(app_state: &AppState, ui: &mut Ui) {
    egui::TopBottomPanel::bottom("visualizer_panel")
        // .resizable(true)
        // .default_height(200.0)
        // .height_range(200.0..=500.0)
        .min_height(300.0)
        .show_inside(ui, |ui| ui.heading("Visualizer panel"));
}
