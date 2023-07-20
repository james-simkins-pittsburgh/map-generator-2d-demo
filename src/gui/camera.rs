use bevy::prelude::*;
pub struct HiveboticaCameraPlugin;

impl Plugin for HiveboticaCameraPlugin {
    fn build(&self, app: &mut App) {
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

fn set_initial_camera(mut main_camera_query: Query<(&mut OrthographicProjection, &mut MainCamera)>) {

    for mut main_camera in main_camera_query.iter_mut() {

    main_camera.0.scale = 8.0;
    }
}



fn camera_pan_and_zoom (keys: Res<Input<KeyCode>>, mut main_camera_query: Query<(&mut Transform, & mut OrthographicProjection, &mut MainCamera)>) {
   

    for mut main_camera in main_camera_query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            main_camera.0.translation.y = main_camera.0.translation.y + crate::PAN_TOP_SPEED * ((main_camera.1.scale / 8.0) + 1.0 )/ 2.0 ;
        }
        if keys.pressed(KeyCode::A) {
            main_camera.0.translation.x = main_camera.0.translation.x - crate::PAN_TOP_SPEED * ((main_camera.1.scale / 8.0) + 1.0 )/ 2.0;
        }
        if keys.pressed(KeyCode::S) {
            main_camera.0.translation.y = main_camera.0.translation.y - crate::PAN_TOP_SPEED * ((main_camera.1.scale / 8.0) + 1.0 )/ 2.0;
        }
        if keys.pressed(KeyCode::D) {
            main_camera.0.translation.x = main_camera.0.translation.x + crate::PAN_TOP_SPEED * ((main_camera.1.scale / 8.0) + 1.0 )/ 2.0;
        }
        if keys.pressed(KeyCode::X) {
            main_camera.1.scale *= 1.0+crate::ZOOM_SPEED;
        }
        if keys.pressed(KeyCode::Z) {
            main_camera.1.scale *= 1.0-crate::ZOOM_SPEED;
        }

        main_camera.1.scale = main_camera.1.scale.clamp (1.0, crate::ZOOM_OUT_MAX);

    }

    
    
}



