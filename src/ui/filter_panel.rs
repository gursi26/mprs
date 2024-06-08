use crate::state::filter_state::F1State;
use crate::ui::toggle_button::toggle;
use crate::F2_PANEL_ROW_HEIGHT;
use crate::{state::state::AppState, utils::f1_state_enum_to_str};
use eframe::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};

// TODO: Dynamically set width

pub fn draw_f1_panel(app_state: &mut AppState, ui: &mut Ui) {
    if app_state.f1_state != app_state.prev_state.f1_state {
        app_state.prev_state.f1_state = app_state.f1_state.clone();
        app_state.f2_state = (*app_state
            .trackdb
            .track_filter_cache
            .get(&app_state.f1_state)
            .unwrap()
            .keys()
            .collect::<Vec<&String>>()
            .get(0)
            .unwrap())
        .clone();
    }

    egui::TopBottomPanel::top("top_left_panel")
        .min_height(150.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            let available_height = 150.0;
            let available_width = ui.available_width();
            ui.vertical_centered(|ui| {
                if ui
                    .add(egui::Button::new("Playlists").min_size(egui::Vec2 {
                        x: available_width,
                        y: available_height / 4.0,
                    }))
                    .clicked()
                {
                    app_state.f1_state = F1State::Playlists;
                };
                if ui
                    .add(egui::Button::new("Artists").min_size(egui::Vec2 {
                        x: available_width,
                        y: available_height / 4.0,
                    }))
                    .clicked()
                {
                    app_state.f1_state = F1State::Artists;
                };
                if ui
                    .add(egui::Button::new("Albums").min_size(egui::Vec2 {
                        x: available_width,
                        y: available_height / 4.0,
                    }))
                    .clicked()
                {
                    app_state.f1_state = F1State::Albums;
                };
                if ui
                    .add(egui::Button::new("All").min_size(egui::Vec2 {
                        x: available_width,
                        y: available_height / 4.0,
                    }))
                    .clicked()
                {
                    app_state.f1_state = F1State::All;
                };
            })
        });
}

pub fn draw_f2_panel(app_state: &mut AppState, ui: &mut Ui) {
    let f2_values = app_state
        .trackdb
        .track_filter_cache
        .get(&app_state.f1_state)
        .unwrap()
        .keys()
        .map(|x| x.clone())
        .collect::<Vec<String>>();

    if app_state.f1_state != app_state.prev_state.f2_state.0
        || app_state.f2_state != app_state.prev_state.f2_state.1
    {
        app_state.prev_state.f2_state.0 = app_state.f1_state.clone();
        app_state.prev_state.f2_state.1 = app_state.f2_state.clone();

        app_state.tracklist_state.empty();
        let tid_iter = match app_state
            .trackdb
            .track_filter_cache
            .get(&app_state.f1_state)
            .unwrap()
            .get(&app_state.f2_state)
        {
            Some(x) => x.iter(),
            None => return,
        };

        for tid in tid_iter {
            let tinfo = app_state.trackdb.trackmap.get(tid).unwrap();
            app_state.tracklist_state.add_item(
                tid.clone(),
                tinfo.name.clone(),
                tinfo.artists.clone().unwrap_or(Vec::new()),
                tinfo.album.clone().unwrap_or(String::new()),
                tinfo.duration,
            )
        }
    }

    egui::CentralPanel::default().show_inside(ui, |ui| {
        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::remainder().clip(true))
            .min_scrolled_height(0.0)
            .max_scroll_height(350.0);

        table = table.sense(egui::Sense::click());
        table.body(|mut body| {
            for curr_row in f2_values.iter() {
                body.row(F2_PANEL_ROW_HEIGHT, |mut row| {
                    if app_state.f2_state == **curr_row {
                        row.set_selected(true);
                    }
                    row.col(|ui| {
                        ui.add(egui::Label::new(curr_row).selectable(false));
                    });

                    let response = row.response();
                    if response.clicked() {
                        app_state.f2_state = (*curr_row).clone();
                    }

                    if let F1State::Playlists = app_state.f1_state {
                        response.context_menu(|ui| {
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
                            ui.menu_button(" Delete Playlist ", |ui| {
                                if ui.button("Confirm").clicked() {
                                    app_state.f2_state = String::from("Liked");
                                    app_state.trackdb.remove_playlist(curr_row);
                                    ui.close_menu();
                                }

                                if ui.button("Cancel").clicked() {
                                    ui.close_menu();
                                }
                            });
                        });
                    }
                });
            }
        });
    });
}
