use std::{time::Duration, f64::consts::PI};

use bevy::prelude::*;

use crate::{mutate_coords, ShootEvent, Velocity, ShootTimer};

const PLAYER_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct Player;

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

        let mutated_player_coords = mutate_coords(transform.translation);
   
        let linear_vec = Vec2::new(1.0, 0.0);
        let player_mouse_vec = Vec2::new(mouse_pos.x - mutated_player_coords.x, mouse_pos.y - mutated_player_coords.y);
        let angle_linear_mouse = linear_vec.angle_between(player_mouse_vec);


        shoot_timer.time.tick(Duration::from_secs_f32(1.0/60.0));
        if shoot_timer.time.elapsed_secs() > 100.0 {
            shoot_timer.time.set_elapsed(Duration::from_secs_f32(5.0));
        }
        if input.pressed(KeyCode::Space) {
            if shoot_timer.time.elapsed_secs() >= 0.25 {
                event.send(ShootEvent {
                    x: transform.translation.x,
                    y: transform.translation.y,
                    vel: Vec2::from_angle(angle_linear_mouse),
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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_controll);
        app.add_system(player_rotation);
    }
}