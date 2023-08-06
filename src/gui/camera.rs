use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use bevy::window::PrimaryWindow;
pub struct HiveboticaCameraPlugin;

impl Plugin for HiveboticaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        app.add_systems(Startup, (camera_setup, set_initial_camera).chain());
        app.add_systems(Update, camera_pan_and_zoom);
    }
}

// This is a marker component for the main camera.

#[derive(Component)]
pub struct MainCamera;

// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn set_initial_camera(
    mut main_camera_query: Query<(&mut OrthographicProjection, &mut MainCamera)>
) {
    for mut main_camera in main_camera_query.iter_mut() {
        main_camera.0.scale = 8.0;
    }
}

fn camera_pan_and_zoom(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut main_camera_query: Query<(&mut Transform, &mut OrthographicProjection, &mut MainCamera)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let mut main_camera = main_camera_query.single_mut();

    // This pans the camera if the cursor position is on the edge of the screen.

    if let Some(position) = window_query.single().cursor_position() {
        if position.y > window_query.single().height() - 50.0 {
            main_camera.0.translation.y =
                main_camera.0.translation.y -
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.y < 50.0 {
            main_camera.0.translation.y =
                main_camera.0.translation.y +
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.x > window_query.single().width() - 50.0 {
            main_camera.0.translation.x =
                main_camera.0.translation.x +
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }

        if position.x < 50.0 {
            main_camera.0.translation.x =
                main_camera.0.translation.x -
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
        }
    }

    // This zooms in or out if the mouse wheel is turned.

    for event in scroll_event_reader.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                if event.y > 0.0 {
                    main_camera.1.scale *= 1.0 - crate::ZOOM_SPEED;
                } else if event.y < 0.0 {
                    main_camera.1.scale *= 1.0 + crate::ZOOM_SPEED;
                }
            }
            MouseScrollUnit::Pixel => {
                if event.y > 0.0 {
                    main_camera.1.scale *= 1.0 - crate::ZOOM_SPEED;
                } else if event.y < 0.0 {
                    main_camera.1.scale *= 1.0 + crate::ZOOM_SPEED;
                }
            }
        }
    }

    main_camera.1.scale = main_camera.1.scale.clamp(1.0, crate::ZOOM_OUT_MAX);
}
