use std::sync::Arc;

use crate::ui::toggle_button::toggle;
use crate::{mpv::play_track, state::state::AppState};
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
        .column(Column::remainder().at_least(600.0).at_most(600.0).clip(true))
        .column(Column::remainder().at_least(200.0).at_most(200.0).resizable(false).clip(true))
        .column(Column::remainder().at_least(200.0).at_most(200.0).resizable(false).clip(true))
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
            let row_height = 30.0;
            for row_index in 0..(app_state.tracklist_state.items.len()) {
                let curr_row = app_state
                    .tracklist_state
                    .items
                    .get(row_index);

                if let None = curr_row {
                    continue;
                }
                let curr_row = curr_row.unwrap().clone();

                body.row(row_height, |mut row| {
                    if let Some(t_id) = app_state.trackqueue.get_curr_track() {
                        if curr_row.id == t_id {
                            row.set_selected(true);
                            row.col(|ui| {
                                ui.label("▶");
                            });
                        } else {
                            row.col(|ui| {
                                ui.label((row_index + 1).to_string());
                            });
                        }
                    } else {
                        row.col(|ui| {
                            ui.label((row_index + 1).to_string());
                        });
                    }

                    row.col(|ui| {
                        ui.label(curr_row.name);
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

                    let response = row.response();
                    if response.clicked() {
                        app_state.trackqueue.empty_reg_queue();

                        let curr_track_ids = app_state
                            .trackdb
                            .track_filter_cache
                            .get(&app_state.f1_state)
                            .unwrap()
                            .get(&app_state.f2_state)
                            .unwrap();

                        for tid in curr_track_ids.iter() {
                            if *tid != curr_row.id {
                                app_state.trackqueue.add_to_reg_queue(tid.clone());
                            }
                        }

                        if app_state.shuffle {
                            app_state.trackqueue.shuffle_reg_queue();
                        }

                        app_state.trackqueue.prepend_to_reg_queue(curr_row.id);
                        app_state.trackqueue.next_track();

                        play_track(app_state);
                    }

                    response.context_menu(|ui| {
                        if ui.button("Play next").clicked() {
                            app_state.trackqueue.play_next(curr_row.id);
                            ui.close_menu();
                        }

                        if ui.button("Add to queue").clicked() {
                            app_state.trackqueue.add_to_queue(curr_row.id);
                            ui.close_menu();
                        }

                        if ui.button("Delete track").clicked() {
                            app_state.trackdb.remove_track(curr_row.id, Some(true));
                            app_state.tracklist_state.remove_with_id(curr_row.id);
                            ui.close_menu();
                        }
                    });
                });
            }
        });
}
