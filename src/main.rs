#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

const WINDOW_WIDHT: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PROJECTILE_SPEED: i32 = 0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Component)]
struct GameState {
    score: i32,
    wave: i32,
}

#[derive(Resource, Component)]
struct GameRules {
    max_enemies: i32,
    boss_phase: bool,
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Health {
    amount: i32,
}

#[derive(Component)]
enum Enemy {
    Legionary,
    Praetorian,
    Penturion,
    Praefectus,
    Tribunus,
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, materials: ResMut<Assets<StandardMaterial>>) {
    
}

fn setup_level(mut commands: Commands) {
   //game Rules
   commands.insert_resource(GameRules{
        
   })

}

fn main() { 
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.89)))
        .add_plugins(DefaultPlugins.set(
                WindowPlugin {
                    window: WindowDescriptor {
                        title: "DNAISC2".to_string(),
                        width: WINDOW_WIDHT,
                        height: WINDOW_HEIGHT,
                        ..default()
                    },
                ..default()
                }
                ))
        .run();
    
}
