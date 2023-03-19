use three_d::egui::*;

use crate::simulation::SimulationContext;

pub fn ui(
    ctx: &Context,
    rotation_speed: &mut f64,
    last_fps: &str,
    elapsed_time: f64,
    sim: &mut SimulationContext,
) {
    SidePanel::left("side_panel")
        .min_width(500.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Bowl")
                        .strong()
                        .size(25.0)
                        .color(Color32::DARK_GREEN),
                );
            });
            ui.separator();

            simulation_section(ui, sim);
            visual_section(ui, rotation_speed);

            TopBottomPanel::bottom("diagnostics").show_inside(ui, |ui| {
                ui.label(format!("Frame time: {:.2}", elapsed_time));
                ui.label(last_fps);
            });
        });
}

fn simulation_section(ui: &mut Ui, sim: &mut SimulationContext) {
    ui.heading("Simulation");
    Grid::new("simulation_configuration").show(ui, |ui| {
        ui.strong("Size");
        ui.end_row();

        ui.label(RichText::new("Dim 1").color(Color32::BLUE));
        ui.add(DragValue::new(&mut sim.dim1_size));
        ui.add(
            ProgressBar::new(sim.state.dim1_fill as f32 / sim.dim1_size as f32).fill(Color32::BLUE),
        );
        ui.end_row();

        ui.label(RichText::new("Dim 2").color(Color32::DARK_GREEN));
        ui.add(DragValue::new(&mut sim.dim2_size));
        ui.add(
            ProgressBar::new(sim.state.dim2_fill as f32 / sim.dim2_size as f32)
                .fill(Color32::DARK_GREEN),
        );
        ui.end_row();

        ui.label(RichText::new("Dim 3").color(Color32::RED));
        ui.add(DragValue::new(&mut sim.dim3_size));
        ui.add(
            ProgressBar::new(sim.state.dim3_fill as f32 / sim.dim3_size as f32).fill(Color32::RED),
        );
        ui.end_row();

        ui.label(RichText::new("Dim 4").color(Color32::YELLOW));
        ui.add(DragValue::new(&mut sim.dim4_size));
        ui.add(
            ProgressBar::new(sim.state.dim4_fill as f32 / sim.dim4_size as f32)
                .fill(Color32::YELLOW),
        );
        ui.end_row();
    });
    Grid::new("simulation_use").show(ui, |ui| {
        ui.strong("Use");
        ui.end_row();
    });
    ui.separator();
}

fn visual_section(ui: &mut Ui, rotation_speed: &mut f64) {
    ui.heading("Visual");
    Grid::new("visual").show(ui, |ui| {
        ui.label("Rotation speed");
        ui.add(Slider::new(rotation_speed, 0.0..=10.0));
        ui.end_row();
    });
}
