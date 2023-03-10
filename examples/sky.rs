use bevy::prelude::*;
use bevy_parallax::{
    LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin,
    ParallaxResource,
};

fn main() {
    // Define window
    let primary_window = Window {
        title: "Sky".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        // Add parallax resource with layer data
        .insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.9, 0.9),
                    path: "sky-stars.png".to_string(),
                    tile_size: Vec2::new(53.0, 55.0),
                    cols: 1,
                    rows: 1,
                    scale: 3.0,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: LayerSpeed::Bidirectional(0.1, 0.5),
                    path: "sky-clouds.png".to_string(),
                    tile_size: Vec2::new(109.0, 43.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.0,
                    z: 1.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                // Use nearest filtering so our pixel art renders clear
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(ParallaxPlugin)
        .add_startup_system(initialize_camera_system)
        .add_system(move_camera_system)
        .run();
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent);
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(3.0, 0.0),
        });
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(-3.0, 0.0),
        });
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, 3.0),
        });
    } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, -3.0),
        });
    }
}
