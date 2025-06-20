use std::{f32::consts::PI, time::Duration};

use bevy::{ prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::{AppState, GlobalTextureAtlas, Player, ENEMIES_SPAWN_INTERVAL, ENEMY_SPEED, NUM_ENEMIES, SPRITE_SCALE_FACTOR};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                (spawn_enemy).run_if(in_state(AppState::InGame)).run_if(on_timer(Duration::from_secs_f32(ENEMIES_SPAWN_INTERVAL))),
                update_enemy_transform
            )
        );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();

    if num_enemies >= NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let enemy_spawn_count = (NUM_ENEMIES - num_enemies).min(10);

    let player_position = player_query.single().unwrap().translation.truncate();

    let mut rng = rand::rng();

    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_position);

        commands.spawn(
        (
                Sprite::from_atlas_image(
                    handle.image.clone().unwrap(),
                    TextureAtlas { 
                        layout: handle.layout.clone().unwrap(), 
                        index: rng.random_range(18..=19)
                    },
                ),
                Transform {
                    translation: vec3(x, y,1.0),
                    scale: Vec3::splat(2.0 * SPRITE_SCALE_FACTOR),
                    ..default()
                },
                Enemy
            )
        );
    }
}


fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_position = player_query.single().unwrap().translation;

    for mut transform in enemy_query.iter_mut() {
        let direction = (player_position - transform.translation).normalize();

        transform.translation += direction * ENEMY_SPEED;
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..PI * 2.0);
    let dist = rng.random_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}
