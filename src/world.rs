use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;
use crate::{AppState, GlobalTextureAtlas, Gun, GunTimer, Player, NUM_WORLD_DECORATION, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInit), (
            init_world,
            spawn_world_decoration
        ));
    }
}



fn init_world (
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut current_state: ResMut<NextState<AppState>>
) {
    commands.spawn(
        (
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas { 
                    layout: handle.layout.clone().unwrap(), 
                    index: 0
                },
            ),
            Transform {
                translation: vec3(0.0, 0.0,1.0),
                scale: Vec3::splat(2.0 * SPRITE_SCALE_FACTOR),
                ..default()
            },
            Player
        )
    );

    commands.spawn(
        (
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas { 
                    layout: handle.layout.clone().unwrap(), 
                    index: 1
                },
            ),
            Transform::from_scale(Vec3::splat(6.0)),
            Gun,
            GunTimer(Stopwatch::new())
        )
    );

    current_state.set(AppState::InGame);
}

fn spawn_world_decoration(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
) {
    let mut rng = rand::rng();

    for _ in 0..NUM_WORLD_DECORATION {
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);

        commands.spawn(
            (
                Sprite::from_atlas_image(
                    handle.image.clone().unwrap(),
                    TextureAtlas { 
                        layout: handle.layout.clone().unwrap(), 
                        index: rng.random_range(3..=17)
                    },
                ),
                Transform {
                    translation: vec3(x, y, 0.0),
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                    ..default()
                },
            )
        );
    }
}
