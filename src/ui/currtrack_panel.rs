use std::sync::Arc;

use crate::ui::toggle_button::toggle;
use crate::utils::duration_to_str;
use crate::{state::state::AppState, utils::get_album_cover};
use eframe::egui::load::Bytes;
use eframe::egui::{self, ImageSource, ProgressBar, Ui};

pub fn draw_currtrack_panel(app_state: &AppState, ui: &mut Ui) {
    let p = app_state.get_curr_track_path();
    if p.is_none() {
        return;
    }
    let p = p.unwrap();

    let uri_str = format!("{}.jpg", app_state.curr_trackinfo.clone().unwrap().name);

    egui::TopBottomPanel::bottom("bottom_left_panel")
        .min_height(350.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            ui.add_space(15.0);
            ui.add(
                egui::Image::new(ImageSource::Bytes {
                    uri: std::borrow::Cow::Owned(uri_str),
                    bytes: Bytes::Shared(Arc::clone(&app_state.curr_albumcover.as_ref().unwrap())),
                })
                .rounding(5.0)
                .max_size(egui::Vec2 { x: 300.0, y: 300.0 }),
            );

            ui.add_space(15.0);

            let s = app_state.curr_trackinfo.clone().unwrap();

            let elapsed_time = app_state.track_clock.elapsed().as_secs_f32();
            let total_time = s.duration as f32;
            let display_text = format!("{}/{}", duration_to_str(elapsed_time as u32), duration_to_str(total_time as u32));

            ui.add(ProgressBar::new(elapsed_time / total_time).text(display_text));

            ui.add_space(5.0);
            ui.strong(&s.name);

            if let Some(artist_names) = s.artists {
                ui.label(artist_names.join(", "));
            }

            if let Some(album_name) = s.album {
                ui.label(album_name);
            }
        });
}
