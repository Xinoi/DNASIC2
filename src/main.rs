#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::time::Stopwatch;
use bevy::window::CursorGrabMode;
use player::*;

mod my_math;
mod player;
mod enemies;

const WINDOW_WIDHT: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const TIME_STEP: f32 = 1.0 / 60.0;

const BULLET_SPEED: f32 = 20.0;

#[derive(Component)]
struct MainCamera;

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
struct ShootTimer {
    time: Stopwatch,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

struct ShootEvent {
    pub x: f32,
    pub y: f32,
    pub vel: Vec2,
    pub rotation: Quat,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
struct Health {
    amount: i32,
}

fn setup() {
    println!("Startet");
}

fn mutate_coords(vec: Vec3) -> Vec2 {
    let w_widht = WINDOW_WIDHT;
    let w_height = WINDOW_WIDHT;
    
    Vec2::new(vec.x + WINDOW_WIDHT/2.0, vec.y + WINDOW_HEIGHT/2.0)
    
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    //game Rules
    commands.insert_resource(GameRules {
        max_enemies: 20,
        boss_phase: false,
    });

    commands.insert_resource(GameState {
        score: 0,
        wave: 1,
    });

    //PlayerShip
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 2.0),
                ..default()
            },
            texture: asset_server.load("ShipTexture.png"),
            ..default()
        },
        Player,
        Health { amount: 100 },
        ShootTimer {
            time: Stopwatch::new(),
        },
        Velocity { x: 0.0, y: 0.0 },
    ));
}

fn spawn_laser(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut shoot_events_reader: EventReader<ShootEvent>,
) {
    for shoot_event in shoot_events_reader.iter() {

        let laser_ent = commands.spawn((
            Laser,
            Velocity {x: shoot_event.vel.x * BULLET_SPEED, y: shoot_event.vel.y * BULLET_SPEED},
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(shoot_event.x, shoot_event.y, 0.0),
                    rotation: shoot_event.rotation,
                    ..default()
                },
                texture: asset_server.load("BlueLaser.png"),
                ..default()
            },
        ));
    }
}

fn move_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn collision_check(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    laser_query: Query<(&Transform, Entity), (With<Laser>, Without<Player>)>
) {
    //Player
    let mut player_trans = player_query.single_mut();
    let fixed_window_x = WINDOW_WIDHT / 2.0;
    let fixed_window_y = WINDOW_HEIGHT / 2.0;
    let player_x = player_trans.translation.x;
    let player_y = player_trans.translation.y;

    //player-borders
    if player_x > fixed_window_x {
        player_trans.translation.x = fixed_window_x;
    } else if player_x < -fixed_window_x {
        player_trans.translation.x = -fixed_window_x;
    } else if player_y > fixed_window_y {
        player_trans.translation.y = fixed_window_y;
    } else if player_y < -fixed_window_y {
        player_trans.translation.y = -fixed_window_y;
    }

    //bullet outer scope

    for (laser_trans, entity) in laser_query.iter() {
        let laser_x = laser_trans.translation.x;
        let laser_y = laser_trans.translation.y;
        if laser_x > fixed_window_x || laser_x < -fixed_window_x || laser_y > fixed_window_y || laser_y < -fixed_window_y {
            commands.entity(entity).despawn();
        }
    }

    
}


fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)))
        .add_event::<ShootEvent>()
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_level)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_system)
                .with_system(collision_check)
                .with_system(spawn_laser),
        )
        .add_system(bevy::window::close_on_esc)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemies::EnemyPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "DNAISC2".to_string(),
                width: WINDOW_WIDHT,
                height: WINDOW_HEIGHT,
                resizable: false,
                cursor_grab_mode: CursorGrabMode::Locked,

                ..default()
            },
            ..default()
        }))
        .run();
}
