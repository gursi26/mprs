use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use crate::mpv::kill_track;
use crate::spotdl::{download_track, search_tracks, SearchResult};
use crate::state::filter_state::F1State;
use crate::state::state::{AppState, AppStateWrapper};
use crate::ui::toggle_button::toggle;
use crate::utils::duration_to_str;
use crate::{NUM_SEARCH_RESULTS, UI_SLEEP_DURATION_MS};
use eframe::egui::{self, Color32, FontData, FontDefinitions, Ui, Window};
use egui_extras::{install_image_loaders, Column, TableBuilder};

use super::{
    currtrack_panel::draw_currtrack_panel,
    filter_panel::{draw_f1_panel, draw_f2_panel},
    tracklist_panel::draw_tracklist,
    visualizer_panel::draw_visualizer,
};

// top panel contains shuffle and light/dark mode toggles
fn draw_top_panel(app_state: &mut AppState, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // TODO: Replace with package name and version number from cargo
            ui.heading("mprs - v0.1.1");
            ui.vertical_centered_justified(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.separator();
                    ui.label("Shuffle");
                    ui.add(toggle(&mut app_state.shuffle));

                    if let F1State::Playlists = app_state.f1_state {
                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::BLACK;

                        ui.separator();
                        ui.menu_button(" Create Playlist ", |ui| {
                            ui.label("New playlist name: ");
                            ui.text_edit_singleline(&mut app_state.new_playlist_name);
                            if ui.button("Create").clicked() {
                                ui.close_menu();
                                app_state
                                    .trackdb
                                    .create_playlist(app_state.new_playlist_name.clone());
                                app_state.new_playlist_name = String::new();
                            }
                        });

                        if app_state.f2_state != "Liked" {
                            ui.menu_button(" Delete Playlist ", |ui| {
                                if ui.button("Confirm").clicked() {
                                    let to_remove = app_state.f2_state.clone();
                                    app_state.f2_state = String::from("Liked");
                                    app_state.trackdb.remove_playlist(to_remove);
                                    ui.close_menu();
                                }

                                if ui.button("Cancel").clicked() {
                                    ui.close_menu();
                                }
                            });
                        }

                        ui.menu_button(" Add Track ", |ui| {
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut app_state.new_track_search_term);
                                if ui.button("Search").clicked() {
                                    let results = search_tracks(
                                        app_state.new_track_search_term.clone(),
                                        NUM_SEARCH_RESULTS,
                                        &mut app_state.spt_creds,
                                    );

                                    app_state.search_results = Some(results);
                                    ui.close_menu();
                                }
                            })
                        });
                    }
                })
            })
        });
    });
}

// bottom panel contains notification screen
fn draw_bottom_panel(app_state: &AppState, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .min_height(20.0)
        .max_height(20.0)
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(&app_state.notification.message);
            })
        });
}

fn draw_left_panel(app_state: &mut AppState, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel")
        .resizable(false)
        // TODO: Set this width to a fraction of the screen size
        .min_width(250.0)
        .show(ctx, |ui| {
            draw_f1_panel(app_state, ui);
            draw_f2_panel(app_state, ui);
            draw_currtrack_panel(app_state, ui);
        });
}

fn draw_main_panel(app_state: &mut AppState, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // draw_visualizer(app_state, ui);
        draw_tracklist(app_state, ui);
    });
}

fn display_search_results_popup(app_state: &mut AppState, ctx: &egui::Context) {
    if let None = app_state.search_results {
        return;
    }
    let search_results = app_state.search_results.clone().unwrap();

    egui::Window::new("Search results")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            if ui.button("Close").clicked() {
                app_state.search_results = None;
            }
            draw_search_result_table(ui, app_state);
        });
}

fn draw_search_result_table(ui: &mut Ui, app_state: &mut AppState) {
    let available_height = ui.available_height();
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

    if app_state.search_results.is_none() {
        return;
    }
    let result_vec = app_state.search_results.clone().unwrap();

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
            for row_index in 0..(result_vec.len()) {
                let curr_row = result_vec.get(row_index);

                if let None = curr_row {
                    continue;
                }
                let curr_row = curr_row.unwrap();

                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.label((row_index + 1).to_string());
                    });
                    row.col(|ui| {
                        ui.label(curr_row.name.clone());
                    });
                    row.col(|ui| {
                        ui.label(curr_row.artists.join(", "));
                    });
                    row.col(|ui| {
                        ui.label(&curr_row.album);
                    });
                    row.col(|ui| {
                        ui.label(duration_to_str(curr_row.duration as u32));
                    });

                    let response = row.response();
                    if response.clicked() {
                        // TODO: Move this bit outside so player does not hang when download in
                        // progress
                        // TODO: Figure out how to live update tracklist after download instead of
                        // having to switch and switch back to curr playlist
                        app_state.search_results = None;
                        download_track(&curr_row.get_url()).wait().unwrap();
                        app_state.trackdb.add_all_tracks(Some(app_state.f2_state.clone()));
                        app_state.ctx.clone().unwrap().request_repaint();
                    }
                });
            }
        });
}

impl eframe::App for AppStateWrapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(UI_SLEEP_DURATION_MS));

        let app_state_clone = Arc::clone(&self.app_state);
        let mut app_state_g = app_state_clone.lock().unwrap();

        if let None = app_state_g.ctx {
            install_image_loaders(ctx);
            app_state_g.ctx = Some(ctx.clone());
        }

        draw_top_panel(&mut app_state_g, ctx);
        draw_bottom_panel(&mut app_state_g, ctx);
        draw_left_panel(&mut app_state_g, ctx);
        draw_main_panel(&mut app_state_g, ctx);
        display_search_results_popup(&mut app_state_g, ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let app_state_clone = Arc::clone(&self.app_state);
        let mut app_state_g = app_state_clone.lock().unwrap();
        match &mut app_state_g.mpv_child {
            Some(mpv_child) => {
                mpv_child.kill().unwrap();
            }
            None => {}
        };

        exit(0);
    }
}
