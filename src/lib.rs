use bevy::{prelude::*, window::WindowMode, winit::WinitSettings};

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                recognize_rotation_gesture: true,
                prefers_home_indicator_hidden: true,
                prefers_status_bar_hidden: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::mobile())
        .insert_resource(ClearColor(Color::linear_rgb(0.9, 0.3, 0.7)))
        .add_systems(Startup, hello)
        .run();
}

fn hello() {
    info!("Hello From Bevy!");
}
