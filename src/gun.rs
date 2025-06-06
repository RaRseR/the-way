use bevy::{prelude::*, time::{Stopwatch, TimerMode}};

use crate::{player::Player, AppState, CursorPosition, GlobalTextureAtlas, BULLET_SPAWN_INTERVAL, BULLET_SPEED, SPRITE_SCALE_FACTOR};

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletDirection(Vec3);

#[derive(Component)]
pub struct Lifetime(Timer);

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_gun_transform, 
            handle_gun_input, 
            update_bullets
        ).run_if(in_state(AppState::InGame)));
    }
}

fn update_gun_transform(
    cursor_position: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_position = player_query.single().unwrap().translation;
    let mut gun_transform = gun_query.single_mut().unwrap();

    let cursor_position = match cursor_position.0 {
        Some(position) => position,
        None => player_position.truncate()
    };

    let to_player = (player_position.truncate() - cursor_position).normalize();
    let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
    gun_transform.rotation = rotate_to_player;
    
    let offset = 50.0;

    let new_gun_position = vec3(player_position.x + offset, player_position.y + offset, player_position.z );

    gun_transform.translation = new_gun_position;
}

fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    handle: Res<GlobalTextureAtlas>
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_timer ) = gun_query.single_mut().unwrap();
    let gun_position = gun_transform.translation.truncate();
    gun_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left)  {
        return;
    }

    let bullet_direction = gun_transform.local_y();

    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        commands.spawn(
            (
                Sprite::from_atlas_image(
                    handle.image.clone().unwrap(),
                    TextureAtlas { 
                        layout: handle.layout.clone().unwrap(), 
                        index: 2
                    },
                ),
                Transform {
                    translation: vec3(gun_position.x, gun_position.y, 1.0),
                    rotation: Quat::from_rotation_z(bullet_direction.x.atan2(bullet_direction.y)),
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR)
                },
                Bullet,
                BulletDirection(*bullet_direction),
                Lifetime(Timer::from_seconds(5.0, TimerMode::Once)),
            )
        );
    }
}

fn update_bullets(
    mut commands: Commands,
    mut bullets_query: Query<(Entity, &mut Transform, &BulletDirection, &mut Lifetime), With<Bullet>>,
    time: Res<Time>,
) {
    if bullets_query.is_empty() {
        return;
    }

    for (bullet, mut transform, direction, mut lifetime) in bullets_query.iter_mut() {
        lifetime.0.tick(time.delta());

        if lifetime.0.finished() {
            commands.entity(bullet).despawn();
        } else {
            transform.translation += direction.0.normalize() * Vec3::splat(BULLET_SPEED);
        }

    }
}