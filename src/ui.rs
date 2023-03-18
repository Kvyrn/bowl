use three_d::egui::*;

pub fn ui(ctx: &Context, rotation_speed: &mut f64, last_fps: &str, elapsed_time: f64) {
    SidePanel::left("side_panel")
        .min_width(500.0)
        .show(ctx, |ui| {
            ui.label(
                RichText::new("Bowl")
                    .strong()
                    .size(25.0)
                    .color(Color32::DARK_GREEN),
            );
            ui.separator();

            ui.heading("Simulation");
            ui.separator();

            ui.heading("Visual");
            ui.label("Rotation speed");
            ui.add(Slider::new(rotation_speed, 0.0..=10.0));

            TopBottomPanel::bottom("diagnostics").show_inside(ui, |ui| {
                ui.label(format!("Frame time: {:.2}", elapsed_time));
                ui.label(last_fps);
            });
        });
}
