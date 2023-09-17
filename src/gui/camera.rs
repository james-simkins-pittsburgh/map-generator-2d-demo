use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use crate::WARP_LENGTH;
use crate::SECTOR_SIZE;
use crate::gui::warp_buttons::WarpSign;

pub struct HiveboticaCameraPlugin;

impl Plugin for HiveboticaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        app.add_systems(Startup, (camera_setup, set_initial_camera).chain());
        app.add_systems(Update, camera_pan_and_zoom);
        app.add_event::<SwitchVisibleSector>();
        app.init_resource::<CameraSectorCoordinates>();
    }
}

#[derive(Event, Default)]
pub struct SwitchVisibleSector;

// This is a marker component for the main camera.

#[derive(Component)]
pub struct MainCamera;

// These are the coordinates of the sector within the coordinate system of the sectors within the game world.
// The coordinates within the sector match the camera translation.

#[derive(Resource, Default)]
pub struct CameraSectorCoordinates {
    pub sector_x: i32,
    pub sector_y: i32,
}

// This spawns the camera using the Bevy default 2D settings.

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// This sets initial camera conditions.

pub fn set_initial_camera(
    mut main_camera_query: Query<(&mut OrthographicProjection, With<MainCamera>)>,
    mut camera_sector_coordinates: ResMut<CameraSectorCoordinates>
) {
    for mut main_camera in main_camera_query.iter_mut() {
        main_camera.0.scale = 8.0;
        main_camera.0.scaling_mode = ScalingMode::AutoMax { max_width: 1920.0, max_height: 1080.0 };

        camera_sector_coordinates.sector_x = 0;
        camera_sector_coordinates.sector_y = 0;
    }
}

// This controls the pan and zoom abilities of the camera.

pub fn camera_pan_and_zoom(
    mut scroll_event_reader: EventReader<MouseWheel>,
    mut main_camera_query: Query<(&mut Transform, &mut OrthographicProjection, With<MainCamera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_sector_coordinates: ResMut<CameraSectorCoordinates>,
    mut writer: EventWriter<SwitchVisibleSector>,
    mut warp_information: ResMut<crate::gui::warp_buttons::WarpInfo>
) {
    for mut main_camera in main_camera_query.iter_mut() {


        // This pans the camera quickly after the warp button has been pressed.

        if warp_information.warping_now {
            if warp_information.warp_timer < WARP_LENGTH {
                warp_information.warp_timer += 1;

                if warp_information.warp_direction.0 == WarpSign::Positive {
                    main_camera.0.translation.x += (((SECTOR_SIZE as f32) * 96.0 * 5.25)/(WARP_LENGTH as f32)).round();
                } else if warp_information.warp_direction.0 == WarpSign::Negative {
                    main_camera.0.translation.x -= ((SECTOR_SIZE as f32) * 96.0 * 5.25)/((WARP_LENGTH as f32)).round();
                } else if warp_information.warp_direction.1 == WarpSign::Positive {
                    main_camera.0.translation.y += ((SECTOR_SIZE as f32) * 96.0 * 5.25)/((WARP_LENGTH as f32)).round();
                } else if warp_information.warp_direction.1 == WarpSign::Negative {
                    main_camera.0.translation.y -= ((SECTOR_SIZE as f32) * 96.0 * 5.25)/((WARP_LENGTH as f32)).round();
                }
            } else {
                warp_information.warping_now = false;
                warp_information.warp_timer = 0;
                warp_information.warp_direction = (WarpSign::Neutral, WarpSign::Neutral);
            }
        } else {
            if warp_information.button_pressed[0] || warp_information.button_pressed[1] {

                warp_information.warping_now = true;
                warp_information.warp_timer = 0;
                warp_information.warp_direction = (WarpSign::Neutral, WarpSign::Positive);

            } else if warp_information.button_pressed[2] || warp_information.button_pressed[3] {

                warp_information.warping_now = true;
                warp_information.warp_timer = 0;
                warp_information.warp_direction = (WarpSign::Positive, WarpSign::Neutral);

            } else if warp_information.button_pressed[4] || warp_information.button_pressed[5] {

                warp_information.warping_now = true;
                warp_information.warp_timer = 0;
                warp_information.warp_direction = (WarpSign::Neutral, WarpSign::Negative);

            } else if warp_information.button_pressed[6] || warp_information.button_pressed[7] {

                warp_information.warping_now = true;
                warp_information.warp_timer = 0;
                warp_information.warp_direction = (WarpSign::Negative, WarpSign::Neutral);

            }
        }

        // This pans the camera if the cursor position is on the edge of the screen.

        if let Some(position) = window_query.single().cursor_position() {
            if position.y > window_query.single().height() - 50.0 {
                main_camera.0.translation.y -=
                    (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
            }

            if position.y < 50.0 {
                main_camera.0.translation.y +=
                    (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
            }

            if position.x > window_query.single().width() - 50.0 {
                main_camera.0.translation.x +=
                    (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
            }

            if position.x < 50.0 {
                main_camera.0.translation.x -=
                    (crate::PAN_TOP_SPEED * (main_camera.1.scale / 8.0 + 1.0)) / 2.0;
            }
        }

        // This adjust the camera if the pan goes outside of the allowed area and switches to the next sector.

        if main_camera.0.translation.x > ((SECTOR_SIZE * 96 * 3) as f32) {
            main_camera.0.translation.x =
                -((SECTOR_SIZE * 96 * 3) as f32) +
                (main_camera.0.translation.x - ((SECTOR_SIZE * 96 * 3) as f32));

            camera_sector_coordinates.sector_x += 1;
            writer.send(SwitchVisibleSector);
        }

        if main_camera.0.translation.x < -((SECTOR_SIZE * 96 * 3) as f32) {
            main_camera.0.translation.x =
                ((SECTOR_SIZE * 96 * 3) as f32) -
                (-((SECTOR_SIZE * 96 * 3) as f32) - main_camera.0.translation.x);

            camera_sector_coordinates.sector_x -= 1;
            writer.send(SwitchVisibleSector);
        }

        if main_camera.0.translation.y > ((SECTOR_SIZE * 96 * 3) as f32) {
            main_camera.0.translation.y =
                -((SECTOR_SIZE * 96 * 3) as f32) +
                (main_camera.0.translation.y - ((SECTOR_SIZE * 96 * 3) as f32));

            camera_sector_coordinates.sector_y += 1;
            writer.send(SwitchVisibleSector);
        }

        if main_camera.0.translation.y < -((SECTOR_SIZE * 96 * 3) as f32) {
            main_camera.0.translation.y =
                ((SECTOR_SIZE * 96 * 3) as f32) -
                (-((SECTOR_SIZE * 96 * 3) as f32) - main_camera.0.translation.y);

            camera_sector_coordinates.sector_y -= 1;
            writer.send(SwitchVisibleSector);
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
}
