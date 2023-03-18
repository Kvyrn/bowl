use three_d::*;

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

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        model.animate(frame_input.accumulated_time as f32);

        for event in frame_input.events.iter() {
            if let Event::KeyPress { kind, handled, .. } = event {
                if !handled && *kind == Key::Q {
                    return FrameOutput {
                        exit: true,
                        ..Default::default()
                    };
                }
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.05, 0.0, 0.1, 1.0, 1.0))
            .render(&camera, &model, &[&light]);

        FrameOutput::default()
    })
}
