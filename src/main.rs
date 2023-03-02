#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

const WINDOW_WIDHT: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PLAYER_SPEED: f32 = 5.0;

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
struct Velocity{
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

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

fn setup() {
   println!("Startet"); 
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
   //game Rules
   commands.insert_resource(GameRules{
        max_enemies: 20,
        boss_phase: false,
   });
    
   //PlayerShip
   commands.spawn((SpriteBundle {
       transform: Transform {
           scale: Vec3::new(2.0, 2.0, 0.0),
           ..default()
       },
       texture: asset_server.load("ShipTexture.png"),
      ..default()
   },
        Player,
        Health{amount: 100},
        Velocity{
            x: 0.0,
            y: 0.0,
        },
   ));
}

fn player_move(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {

    let mut vel = query.single_mut();
    let mut dir_x = 0.0;
    let mut dir_y = 0.0;


    if input.pressed(KeyCode::W) {
        dir_y = 1.0;
    }
    if input.pressed(KeyCode::S) {
        dir_y = -1.0;
    }
    if input.pressed(KeyCode::D) {
        dir_x = 1.0;
    }
    if input.pressed(KeyCode::A) {
        dir_x = -1.0;
    }

    vel.x = dir_x * PLAYER_SPEED;
    vel.y = dir_y * PLAYER_SPEED;
    

}

fn move_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
    
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() { 
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)))
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_level)
        .add_system(move_system)
        .add_system(player_move)
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
