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
use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, Layout, Ui, Vec2, ViewportInfo, Visuals, Window
};
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
            ui.heading(format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ));
            ui.vertical_centered_justified(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.separator();
                    ui.label("Shuffle");
                    ui.add(toggle(&mut app_state.shuffle));

                    if let F1State::Playlists = app_state.f1_state {
                        ui.separator();
                        if ui.button(" Add Track ").clicked() {
                            app_state.search_results = Some(Vec::new());
                        };
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

    egui::Window::new("Search results")
        .resizable(false)
        .collapsible(false)
        .min_height(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut app_state.new_track_search_term);
                if ui.button("Search").clicked() {
                    let results = search_tracks(
                        app_state.new_track_search_term.clone(),
                        NUM_SEARCH_RESULTS,
                        &mut app_state.spt_creds,
                    );
                    app_state.search_results = Some(results);
                }

                if ui.button("Download").clicked() {
                    app_state.search_results = None;
                    app_state.pending_download_childs.0 = app_state.f2_state.clone();
                    for v in app_state.selected_result_urls.values() {
                        let child = download_track(v);
                        app_state.pending_download_childs.1.push(child);
                    }
                    app_state.selected_result_urls.clear();
                    app_state.notification.set_message(
                        format!(
                            "Downloading tracks, {} remaining...",
                            app_state.pending_download_childs.1.len()
                        ),
                        None,
                    );
                }
                if ui.button("Close").clicked() {
                    app_state.search_results = None;
                }
            });

            draw_search_result_table(ui, app_state);
        });
}

fn draw_search_result_table(ui: &mut Ui, app_state: &mut AppState) {
    if app_state.search_results.is_none() {
        return;
    }
    let result_vec = app_state.search_results.clone().unwrap();
    if result_vec.is_empty() {
        return;
    }

    let available_height = ui.available_height();
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().at_least(20.0))
        .column(
            Column::remainder()
                .at_least(600.0)
                .at_most(600.0)
                .clip(true),
        )
        .column(
            Column::remainder()
                .at_least(200.0)
                .at_most(200.0)
                .resizable(false)
                .clip(true),
        )
        .column(
            Column::remainder()
                .at_least(200.0)
                .at_most(200.0)
                .resizable(false)
                .clip(true),
        )
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
            for row_index in 0..(result_vec.len()) {
                let curr_row = result_vec.get(row_index);

                if let None = curr_row {
                    continue;
                }
                let curr_row = curr_row.unwrap();

                body.row(row_height, |mut row| {
                    if app_state.selected_result_urls.contains_key(&row_index) {
                        row.set_selected(true);
                    }

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
                        if app_state.selected_result_urls.contains_key(&row_index) {
                            app_state.selected_result_urls.remove_entry(&row_index);
                        } else {
                            app_state
                                .selected_result_urls
                                .insert(row_index, curr_row.get_url());
                        }
                    }
                });
            }
        });
}

pub fn check_download_progress(app_state: &mut AppState) {
    let mut remove_idxs = Vec::new();
    for (i, c) in app_state.pending_download_childs.1.iter_mut().enumerate() {
        if c.try_wait().unwrap().is_some() {
            remove_idxs.push(i);
        }
    }

    for idx in remove_idxs.into_iter().rev() {
        app_state.pending_download_childs.1.remove(idx);
    }

    if app_state.pending_download_childs.1.len() > 0 {
        app_state.notification.set_message(
            format!(
                "Downloading tracks, {} remaining...",
                app_state.pending_download_childs.1.len()
            ),
            None,
        );
    } else {
        if app_state.pending_download_childs.0.len() > 0 {
            app_state
                .notification
                .set_message("All tracks downloaded".to_string(), Some(3));
            app_state
                .trackdb
                .add_all_tracks(Some(app_state.pending_download_childs.0.clone()));
            app_state.pending_download_childs = (String::new(), Vec::new());

            let ids = app_state.get_curr_displayed_tracklist();

            let present_ids = app_state
                .tracklist_state
                .items
                .iter()
                .map(|x| x.id)
                .collect::<Vec<u32>>();
            if ids.len() != present_ids.len() {
                for id in ids.iter() {
                    if !present_ids.contains(id) {
                        let tinfo = app_state.trackdb.trackmap.get(id).unwrap();
                        app_state.tracklist_state.add_item(
                            id.clone(),
                            tinfo.name.clone(),
                            tinfo.artists.clone().unwrap_or(Vec::new()),
                            tinfo.album.clone().unwrap_or(String::new()),
                            tinfo.duration,
                        );
                    }
                }
            }

            app_state.ctx.clone().unwrap().request_repaint();
        }
    }
}

pub fn setup_fn(app_state: &mut AppState, ctx: &egui::Context) {
    if let None = app_state.ctx {
        install_image_loaders(ctx);
        app_state.ctx = Some(ctx.clone());
    }
}

pub fn update_shuffle(app_state: &mut AppState) {
    if app_state.shuffle != app_state.prev_state.shuffle {
        if app_state.shuffle {
            app_state.trackqueue.shuffle_reg_queue();
        } else {
            app_state
                .trackqueue
                .add_ordered_tracklist_to_reg_queue(app_state.get_curr_displayed_tracklist());
        }
        app_state.prev_state.shuffle = app_state.shuffle;
    }
}

impl eframe::App for AppStateWrapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(UI_SLEEP_DURATION_MS));

        let app_state_clone = Arc::clone(&self.app_state);
        let mut app_state_g = app_state_clone.lock().unwrap();

        setup_fn(&mut app_state_g, ctx);

        draw_top_panel(&mut app_state_g, ctx);
        draw_left_panel(&mut app_state_g, ctx);
        draw_bottom_panel(&mut app_state_g, ctx);
        draw_main_panel(&mut app_state_g, ctx);
        display_search_results_popup(&mut app_state_g, ctx);

        update_shuffle(&mut app_state_g);
        check_download_progress(&mut app_state_g);
        app_state_g.notification.update_message();
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
