use bevy::{prelude::*, winit::WinitSettings};

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                recognize_rotation_gesture: true,
                prefers_home_indicator_hidden: true,
                prefers_status_bar_hidden: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::mobile())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("character.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
