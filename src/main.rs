use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 700.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d::default(),
    ));
}