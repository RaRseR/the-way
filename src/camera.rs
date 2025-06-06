use bevy::prelude::*;

use crate::{AppState, Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
         app
         .add_systems(OnEnter(AppState::GameInit), (setup_camera,))
         .add_systems(Update, (camera_follow_player,).run_if(in_state(AppState::InGame)));
    }
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query:  Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut().unwrap();
    let player_transform = player_query.single().unwrap().translation;

    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}