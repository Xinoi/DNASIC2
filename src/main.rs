use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.20, 0.33, 1.00)))
        .add_plugins(DefaultPlugins)
        .run();
    
}
