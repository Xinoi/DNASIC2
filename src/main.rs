use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    //Background Image
    todo!();

}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.20, 0.33, 1.00)))
        .add_plugins(DefaultPlugins.set(
                WindowPlugin {
                    window: WindowDescriptor {
                        title: "DNAISC2".to_string(),
                        width: 1200.,
                        height: 800.,
                        ..default()
                    },
                ..default()
                }
                ))
        .run();
    
}
