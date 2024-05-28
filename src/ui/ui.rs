use crate::state::state::AppState;
use crate::ui::toggle_button::toggle;
use eframe::egui;

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

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        draw_top_panel(self, ctx);
        draw_bottom_panel(self, ctx);

        egui::SidePanel::left("left_panel")
            .resizable(false)
            // TODO: Set this width to a fraction of the screen size
            .min_width(250.0)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("top_left_panel")
                    // TODO: Same width thing
                    .min_height(150.0)
                    .resizable(false)
                    .show_inside(ui, |ui| ui.heading("Top left panel"));

                egui::TopBottomPanel::bottom("bottom_left_panel")
                    // TODO: Same width thing
                    .min_height(350.0)
                    .resizable(false)
                    .show_inside(ui, |ui| ui.heading("bottom left panel"));

                egui::CentralPanel::default().show_inside(ui, |ui| ui.heading("Bottom left panel"));
                // egui::ScrollArea::vertical().show(ui, |ui| {});
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::bottom("visualizer_panel")
                // .resizable(true)
                // .default_height(200.0)
                // .height_range(200.0..=500.0)
                .min_height(300.0)
                .show_inside(ui, |ui| ui.heading("Visualizer panel"));

            egui::CentralPanel::default().show_inside(ui, |ui| ui.heading("Main track panel"))
        });
    }
}
