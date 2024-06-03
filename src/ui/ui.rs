use std::process::exit;
use std::sync::Arc;

use crate::mpv::kill_track;
use crate::state::state::{AppState, AppStateWrapper};
use crate::ui::toggle_button::toggle;
use eframe::egui::{self, Ui};

use super::{
    currtrack_panel::draw_currtrack_panel, filter_panel::{draw_f1_panel, draw_f2_panel}, tracklist_panel::draw_tracklist, visualizer_panel::draw_visualizer
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
        .min_width(350.0)
        .show(ctx, |ui| {
            draw_f1_panel(app_state, ui);
            draw_f2_panel(app_state, ui);
            draw_currtrack_panel(app_state, ui);
        });
}

fn draw_main_panel(app_state: &mut AppState, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_visualizer(app_state, ui);
        draw_tracklist(app_state, ui);
    });
}

impl eframe::App for AppStateWrapper {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let app_state_clone = Arc::clone(&self.app_state);
        let mut app_state_g = app_state_clone.lock().unwrap();

        draw_top_panel(&mut app_state_g, ctx);
        draw_bottom_panel(&mut app_state_g, ctx);
        draw_left_panel(&mut app_state_g, ctx);
        draw_main_panel(&mut app_state_g, ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let app_state_clone = Arc::clone(&self.app_state);
        let mut app_state_g = app_state_clone.lock().unwrap();
        match &mut app_state_g.mpv_child {
            Some(mpv_child) => {
                mpv_child.kill().unwrap();
            }
            None => {
            }
        };

        exit(0);
    }
}
