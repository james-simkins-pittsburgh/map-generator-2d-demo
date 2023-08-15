use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use crate::SECTOR_SIZE;
pub struct HiveboticaCameraPlugin;

impl Plugin for HiveboticaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        app.add_systems(Startup, (camera_setup, set_initial_camera).chain());
        app.add_systems(Update, camera_pan_and_zoom);
        app.init_resource::<CameraWorldCoordinates>();
    }
}

// This is a marker component for the main camera.

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct CameraWorldCoordinates {
    pub world_x: i64,
    pub world_y: i64,
}

// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn set_initial_camera(
    mut main_camera_query: Query<(&mut OrthographicProjection, With<MainCamera>)>,
    mut camera_world_coordinates: ResMut<CameraWorldCoordinates>
) {
    let mut main_camera = main_camera_query.single_mut();
    main_camera.0.scale = 8.0;
    main_camera.0.scaling_mode = ScalingMode::AutoMax { max_width: 1920.0, max_height: 1080.0 };

    camera_world_coordinates.world_x = 0;
    camera_world_coordinates.world_y = 0;
}

fn camera_pan_and_zoom(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut main_camera_query: Query<(&mut Transform, &mut OrthographicProjection, &mut MainCamera)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_world_coordinates: ResMut<CameraWorldCoordinates>
) {
    let mut main_camera = main_camera_query.single_mut();

    // This pans the camera if the cursor position is on the edge of the screen.

    if let Some(position) = window_query.single().cursor_position() {
        if position.y > window_query.single().height() - 50.0 {
            main_camera.0.translation.y -= (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round();

            camera_world_coordinates.world_y -= (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round() as i64;
        }

        if position.y < 50.0 {
            main_camera.0.translation.y += (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round();

            camera_world_coordinates.world_y += (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round() as i64;
        }

        if position.x > window_query.single().width() - 50.0 {
            main_camera.0.translation.x += (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round();

            camera_world_coordinates.world_x += (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round() as i64;
        }

        if position.x < 50.0 {
            main_camera.0.translation.x -= (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round();

            camera_world_coordinates.world_x -= (
                (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) /
                2.0
            ).round() as i64;
        }
    }


    // This adjust the camera if the pan goes outside of the allowed area.


    if main_camera.0.translation.x > ((SECTOR_SIZE * 96 * 3) as f32) {
        main_camera.0.translation.x =
            -((SECTOR_SIZE * 96 * 3) as f32) +
            (main_camera.0.translation.x - ((SECTOR_SIZE * 96 * 3) as f32));
    }

    if main_camera.0.translation.x < -((SECTOR_SIZE * 96 * 3) as f32) {
        main_camera.0.translation.x =
            ((SECTOR_SIZE * 96 * 3) as f32) -
            (((SECTOR_SIZE * 96 * 3) as f32) - main_camera.0.translation.x);
    }

    if main_camera.0.translation.y > ((SECTOR_SIZE * 96 * 3) as f32) {
        main_camera.0.translation.y =
            -((SECTOR_SIZE * 96 * 3) as f32) +
            (main_camera.0.translation.y - ((SECTOR_SIZE * 96 * 3) as f32));
    }

    if main_camera.0.translation.y < -((SECTOR_SIZE * 96 * 3) as f32) {
        main_camera.0.translation.y =
            ((SECTOR_SIZE * 96 * 3) as f32) -
            (((SECTOR_SIZE * 96 * 3) as f32) - main_camera.0.translation.y);
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
