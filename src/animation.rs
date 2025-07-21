use bevy::prelude::*;

use crate::Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        //test
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas), With<Player>>
) {
    if player_query.is_empty() {
        return;
    }

    let texture = player_query.single_mut().unwrap();

}