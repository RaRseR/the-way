use bevy::prelude::*;

use crate::{AppState, PLAYER_SPEED};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_player_input).run_if(in_state(AppState::InGame)));
    }
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut().unwrap();

    let w_key = keyboard_input.pressed(KeyCode::KeyW);
    let a_key = keyboard_input.pressed(KeyCode::KeyA);
    let s_key = keyboard_input.pressed(KeyCode::KeyS);
    let d_key = keyboard_input.pressed(KeyCode::KeyD);

    let mut delta = Vec2::ZERO;

    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }

    delta = delta.normalize();
    if delta.is_finite() {
        transform.translation += vec3(delta.x * PLAYER_SPEED, delta.y * PLAYER_SPEED, 0.0) ;
    }
}