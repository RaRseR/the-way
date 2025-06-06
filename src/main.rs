
use bevy::prelude::*;
use the_way::*;

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
        .init_state::<AppState>()
        .add_plugins(ResourcePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}



