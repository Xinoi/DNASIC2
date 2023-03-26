use bevy::prelude::*;
use std::collections::HashMap;

use crate::{GameState, GameRules, WINDOW_WIDHT, WINDOW_HEIGHT, Laser, Health, Velocity};


//health_amount[leagionary, praetorian]
pub const HEALTH_AMOUNT: [i32; 2] = [100, 500];

#[derive(Component)]
struct Enemy {
    enemy_type: EnemieType,
}

#[derive(Clone, Debug)]
struct SpawnPoint {
    x: f32,
    y: f32,
    empty: bool,
}
impl SpawnPoint {
    fn new(new_x: f32, new_y: f32) -> Self {
        Self {
            x: new_x,
            y: new_y,
            empty: true,
        }
    }

    fn from_vec2(vector2: Vec2) -> Self {
        Self {
            x: vector2.x,
            y: vector2.y,
            empty: true,
        }
    }

    fn in_radius(&self, in_x: f32, in_y: f32) -> bool{
        if in_x > (self.x - 10.0) && in_x < (self.x + 10.0) ||  in_y > (self.y - 10.0) && in_y < (self.y + 10.0) {
            true
        }else {
            false
        }
    }

    fn in_vec_radius(&self, in_coords: Vec3) -> bool{
        if in_coords.x > (self.x - 10.0) && in_coords.x < (self.x + 10.0) ||  in_coords.y > (self.y - 10.0) && in_coords.y < (self.y + 10.0) {
            true
        }else {
            false
        }
    }

    fn get_vec3 (&self) -> Vec3 {
        Vec3::new(self.x, self.y, 1.0)
    }
}

enum EnemieType {
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

fn get_free_spawnpoints (query: &Query<&Transform, Without<Laser>>) -> Vec<SpawnPoint>{

    let window_tenth: (f32, f32) = (WINDOW_WIDHT/10.0, WINDOW_HEIGHT/10.0);
    let mut all_spawn_points: Vec<SpawnPoint> = Vec::new();
    //fill SpawnPoint Array
    for i in (1..10).rev() {
        for j in (1..10).rev() {
            all_spawn_points.push(SpawnPoint::new(window_tenth.0 * i as f32, window_tenth.1 * j as f32));
        }
    }

    let mut free_spawn_points: Vec<SpawnPoint> = all_spawn_points.clone();
    for pos in query {
        for i in all_spawn_points.iter_mut().enumerate() {
            if i.1.in_vec_radius(pos.translation) {
                free_spawn_points[i.0].empty = false;
            }
        }
    }

    free_spawn_points

}

fn spawn (
    pos_query: Query<&Transform, Without<Laser>>,
    mut commands: Commands,
    gamerules: Res<GameRules>,
    gamestate: Res<GameState>,
    asset_server: Res<AssetServer>
) {

    let enemy_count = get_enemy_count(gamestate.wave);
    let free_points = get_free_spawnpoints(&pos_query);

    //queue for enemys that should spawn -> Hashmap: [enemy_id, enemytype: (0:normal, 1:praetorian)]
    let mut enemy_queue = HashMap::new();

    if enemy_count.2 == true {
        //boss_phase
    }else {
        for i in 0..enemy_count.0 {
            enemy_queue.insert(i, 0);
        }
        for i in 0..enemy_count.1 as i32 {
            if let Some(key) = enemy_queue.get_mut(&i) {
                *key = 1;
            }
        }
    }

    for entry in enemy_queue.iter().enumerate() {
        
        if *entry.1.1 == 0 {
            commands.spawn((
            SpriteBundle {
                texture: asset_server.load("legionary.png"),
                transform: Transform { 
                    translation: free_points[entry.0].get_vec3(),
                    ..default()
                },
                ..default()
            },
            Enemy {
                enemy_type: EnemieType::Legionary,
            },
            Health { amount: HEALTH_AMOUNT[0]},
            Velocity { x: 0.0, y: 0.0 },
        
        ));
        }

        if *entry.1.1 == 1 {
            commands.spawn((
            SpriteBundle {
                texture: asset_server.load("praetorian.png"),
                transform: Transform { 
                    translation: free_points[entry.0].get_vec3(),
                    ..default()
                },
                ..default()
            },
            Enemy {
                enemy_type: EnemieType::Legionary,
            },
            Health { amount: HEALTH_AMOUNT[1]},
            Velocity { x: 0.0, y: 0.0 },
        
        ));
        }

    }

}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn);
    }
}
