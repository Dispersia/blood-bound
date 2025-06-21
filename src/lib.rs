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
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("character.png");

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(texture),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Single<&mut Transform, With<Player>>,
) {
    let speed = 300.0;

    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    query.translation += direction * speed * time.delta_secs();
}
