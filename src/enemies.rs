use bevy::prelude::*;

use crate::{GameState, GameRules, my_math::Vector2D};

#[derive(Component)]
enum Enemy {
    Legionary,
    Praetorian,
    Tribunus,
}

//returns how many enemies and how strong they should be according to the wave
fn get_enemy_count (wave: i32) -> (i32, f32, bool) {
    if wave % 5 == 0 {
        return (0, 0.0, true);
    }

    let normal_count = if wave < 10 {
        wave
    } else {
        10
    };

    let special_perc = wave as f32 / 100.0;

    (normal_count, special_perc, false)

}

fn set_spawnpoints () -> Vec<Vector2D> {
    todo!();
}

fn spawn_enemy (
    commands: &mut Commands,
    gamerules: Res<GameRules>,
    gamestate: Res<GameState>,
) {

    let enemies = get_enemy_count(gamestate.wave);
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {

    }
}
