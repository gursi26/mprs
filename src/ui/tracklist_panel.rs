use crate::state::state::AppState;
use crate::ui::toggle_button::toggle;
use eframe::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};

pub fn draw_tracklist(app_state: &mut AppState, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        table_ui(app_state, ui);
    });
}

fn table_ui(app_state: &mut AppState, ui: &mut egui::Ui) {
    let available_height = ui.available_height();
    // TODO: Maybe dont hardcode size values in table (use percentage of screen size)
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().at_least(20.0))
        .column(Column::remainder().at_least(600.0))
        .column(Column::remainder().at_least(200.0).resizable(false))
        .column(Column::remainder().at_least(200.0).resizable(false))
        .column(Column::remainder())
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);

    table = table.sense(egui::Sense::click());
    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("");
            });
            header.col(|ui| {
                ui.strong("Name");
            });
            header.col(|ui| {
                ui.strong("Artist");
            });
            header.col(|ui| {
                ui.strong("Album");
            });
            header.col(|ui| {
                ui.strong("Duration");
            });
        })
        .body(|mut body| {
            // TODO: Put this in a constant?
            let row_height = 18.0;
            for row_index in 0..(app_state.tracklist_state.items.len()) {
                let curr_row = app_state.tracklist_state.items.get(row_index).unwrap();
                body.row(row_height, |mut row| {
                    // use this to highlight row for currently playing track
                    // row.set_selected(self.selection.contains(&row_index));
                    row.col(|ui| {
                        ui.label((row_index + 1).to_string());
                    });
                    row.col(|ui| {
                        ui.label(&curr_row.name);
                    });
                    row.col(|ui| {
                        ui.label(&curr_row.artist);
                    });
                    row.col(|ui| {
                        ui.label(&curr_row.album);
                    });
                    row.col(|ui| {
                        ui.label(&curr_row.duration);
                    });

                    // if row.response().clicked() {
                    //     // do something if this row is clicked
                    // }
                });
            }

        });
}
