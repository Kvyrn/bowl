use std::time::Duration;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const UPDATE_INTERVAL: Duration = Duration::from_millis(500);
const FONT_SIZE: f32 = 20.0;

pub struct DiagsPlugin;

impl Plugin for DiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_diags)
            .add_system(update_fps)
            .init_resource::<DiagsState>();
    }
}

#[derive(Resource)]
pub struct DiagsState {
    pub timer: Timer,
    pub update_now: bool,
}

impl Default for DiagsState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
    }
}

#[derive(Component)]
pub struct DiagsText;

fn update_fps(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut state: ResMut<DiagsState>,
    mut text_query: Query<&mut Text, With<DiagsText>>,
) {
    if state.timer.tick(time.delta()).just_finished() || state.update_now {
        state.update_now = false;
        let Some(fps) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.value()) else {return;};
        let fps = format!("{:.2}", fps);

        for mut text in text_query.iter_mut() {
            let value = &mut text.sections[1].value;
            *value = fps.clone();
        }
    }
}

fn setup_diags(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Anonymous_Pro.ttf");
    let style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: Color::ALICE_BLUE,
    };
    commands
        .spawn((
            TextBundle::from_sections([
                TextSection::new("FPS: ", style.clone()),
                TextSection::new("...", style),
            ]),
            DiagsText,
        ))
        // Override visibility
        .insert(Visibility::Hidden);
}
