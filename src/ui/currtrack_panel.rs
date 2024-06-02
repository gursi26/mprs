use crate::state::state::AppState;
use crate::ui::toggle_button::toggle;
use eframe::egui::{self, Ui};

pub fn draw_currtrack_panel(app_state: &AppState, ui: &mut Ui) {
    egui::TopBottomPanel::bottom("bottom_left_panel")
        .min_height(350.0)
        .resizable(false)
        .show_inside(ui, |ui| {
        });
}
