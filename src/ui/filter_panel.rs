use crate::state::filter_state::F1State;
use crate::ui::toggle_button::toggle;
use crate::{state::state::AppState, utils::f1_state_enum_to_str};
use eframe::egui::{self, Ui};

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
            ui.vertical(|ui| {
                ui.selectable_value(&mut app_state.f1_state, F1State::All, "All");
                ui.selectable_value(&mut app_state.f1_state, F1State::Playlists, "Playlists");
                ui.selectable_value(&mut app_state.f1_state, F1State::Artists, "Artists");
                ui.selectable_value(&mut app_state.f1_state, F1State::Albums, "Albums");
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
        .collect::<Vec<&String>>();

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
            .get(&app_state.f2_state) {
                Some(x) => x.iter(),
                None => return 
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
        egui::ScrollArea::vertical().show(ui, |ui| {
            for s in f2_values.iter() {
                let v = (*s).clone();
                ui.selectable_value(&mut app_state.f2_state, v.clone(), v);
            }
        })
    });
}
