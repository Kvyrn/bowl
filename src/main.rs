use three_d::*;

const FPS_UPDATE_RATE: u32 = 10;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Bowl!".to_string(),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 1.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut model = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color::new_opaque(0, 150, 0),
                metallic: 0.5,
                roughness: 0.5,
                ..Default::default()
            },
        ),
    );
    model.set_transformation(Mat4::from_scale(0.2));
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.001)));

    let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));

    let mut gui = three_d::GUI::new(&context);

    let mut last_fps_update = 0u32;
    let mut last_fps = String::new();
    window.render_loop(move |mut frame_input| {
        last_fps_update += 1;
        if last_fps_update >= FPS_UPDATE_RATE {
            last_fps = format!("FPS: {:.4}", 1000.0 / frame_input.elapsed_time);
            last_fps_update = 0;
        }

        // camera.set_viewport(frame_input.viewport);
        model.animate(frame_input.accumulated_time as f32);

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                use three_d::egui::*;
                SidePanel::left("side_panel")
                    .min_width(500.0)
                    .show(ctx, |ui| {
                        ui.heading("Bowl");
                        ui.separator();
                        TopBottomPanel::bottom("diagnostics").show_inside(ui, |ui| {
                            ui.label(format!("Frame time: {:.4}", frame_input.elapsed_time));
                            ui.label(&last_fps);
                        });
                    });

                panel_width = ctx.used_rect().width() as f64;
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.05, 0.0, 0.1, 1.0, 1.0))
            .render(&camera, &model, &[&light])
            .write(|| gui.render());

        FrameOutput::default()
    });
}
