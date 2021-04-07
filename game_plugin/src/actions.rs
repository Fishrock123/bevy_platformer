use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions.system()),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    let mut player_movement = actions.player_movement.unwrap_or(Vec2::ZERO);

    player_movement.y = 0.;
    player_movement.x = 0.;

    if keyboard_input.pressed(KeyCode::W) {
        player_movement.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player_movement.y += -1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_movement.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        player_movement.x += -1.;
    }

    if player_movement == Vec2::ZERO {
        actions.player_movement = None;
    } else {
        player_movement = player_movement.normalize();
        actions.player_movement = Some(player_movement);
    }
}
