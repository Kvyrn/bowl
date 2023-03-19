mod simulation;
mod ui;

use crate::simulation::{Dim, SimulationContext};
use crate::ui::ui;
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
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.0001)));

    let light = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));

    let mut gui = three_d::GUI::new(&context);

    let mut roataion_speed = 1.0;

    let mut sim = SimulationContext::new(3, 5, 7, 10, Dim::D4);

    let mut last_fps_update = 0u32;
    let mut last_fps = String::new();
    window.render_loop(move |mut frame_input| {
        last_fps_update += 1;
        if last_fps_update >= FPS_UPDATE_RATE {
            last_fps = format!("FPS: {:.0}", 1000.0 / frame_input.elapsed_time);
            last_fps_update = 0;
        }

        // camera.set_viewport(frame_input.viewport);
        model.animate((frame_input.accumulated_time * roataion_speed) as f32);

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                ui(
                    ctx,
                    &mut roataion_speed,
                    &last_fps,
                    frame_input.elapsed_time,
                    &mut sim,
                );

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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
    Ok(())
}
