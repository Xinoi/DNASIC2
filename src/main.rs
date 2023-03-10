#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::time::Stopwatch;
use std::time::Duration;
use std::f64::consts::PI;

mod my_math;

const WINDOW_WIDHT: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 10.0;

const X_LINEAR: Vec2 = Vec2::new(1.0, 0.0);

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

#[derive(Component)]
struct Player;

struct ShootEvent {
    pub x: f32,
    pub y: f32,
    pub vel: Vec2,
    pub rotation: Quat,
}

#[derive(Component)]
struct Laser;

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
        
        commands.spawn((
            Laser,
            Velocity {x: shoot_event.vel.x, y: shoot_event.vel.y},
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(shoot_event.x, shoot_event.y, 1.0),
                    rotation: shoot_event.rotation,
                    ..default()
                },
                texture: asset_server.load("BlueLaser.png"),
                ..default()
            },
        ));
    }
}

fn player_controll(
    mut event: EventWriter<ShootEvent>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    mut shoot_query: Query<&mut ShootTimer>,
    windows: Res<Windows>,

) {
    let win = windows.get_primary().expect("no primary window");

    let mouse_pos: Vec2 = match win.cursor_position() {
        Some(num) => num,
        None => Vec2::new(0.0, 0.0),
    };

    let mut shoot_timer = shoot_query.single_mut();
    for (mut velocity, transform) in &mut query {
        let mut dir_x = 0.0;
        let mut dir_y = 0.0;

        let axis: (Vec3, f32) = transform.rotation.to_axis_angle();

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

        if input.pressed(KeyCode::Space) {
           shoot_timer.time.tick(Duration::from_secs_f32(1.0/60.0));
            if shoot_timer.time.elapsed_secs() > 0.5 {
                event.send(ShootEvent {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    vel: Vec2::from_angle(transform.rotation.z).rotate(X_LINEAR),
                    rotation: transform.rotation,
                });
                shoot_timer.time.reset();
            }
        }
        velocity.x = dir_x * PLAYER_SPEED;
        velocity.y = dir_y * PLAYER_SPEED;
    }
}

fn player_rotation(
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Res<Windows>,
) {
    let mut transform = player_query.single_mut();
    let win = windows.get_primary().expect("no primary window");
    let mouse_pos: Vec2 = match win.cursor_position() {
        Some(num) => num,
        None => Vec2::new(0.0, 0.0),
    };
    
    //transform.looking_at() would have done it too ... but i saw it too late :(

    let mutated_player_coords = mutate_coords(transform.translation);
   
    let linear_vec = Vec2::new(1.0, 0.0);
    let player_mouse_vec = Vec2::new(mouse_pos.x - mutated_player_coords.x, mouse_pos.y - mutated_player_coords.y);
    let angle_linear_mouse = linear_vec.angle_between(player_mouse_vec);
    
    transform.rotation = Quat::from_rotation_z(angle_linear_mouse - (PI as f32/2.0));

}

fn move_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn collision_check(mut player_query: Query<&mut Transform, With<Player>>) {
    //Player
    let mut player_trans = player_query.single_mut();
    let fixed_window_x = WINDOW_WIDHT / 2.0;
    let fixed_window_y = WINDOW_HEIGHT / 2.0;
    let player_x = player_trans.translation.x;
    let player_y = player_trans.translation.y;

    if player_x > fixed_window_x {
        player_trans.translation.x = fixed_window_x;
    } else if player_x < -fixed_window_x {
        player_trans.translation.x = -fixed_window_x;
    } else if player_y > fixed_window_y {
        player_trans.translation.y = fixed_window_y;
    } else if player_y < -fixed_window_y {
        player_trans.translation.y = -fixed_window_y;
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
                .with_system(player_controll)
                .with_system(move_system)
                .with_system(collision_check)
                .with_system(spawn_laser),
        )
        .add_system(player_rotation)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "DNAISC2".to_string(),
                width: WINDOW_WIDHT,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .run();
}
