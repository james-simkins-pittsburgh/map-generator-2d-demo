use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub(super) mod camera;
pub(super) mod warp_buttons;

pub struct HiveboticaGUIPluginGroup;

impl PluginGroup for HiveboticaGUIPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(camera::HiveboticaCameraPlugin).add(HiveboticaGUIStartupPlugin). add(warp_buttons::HiveboticaWarpButtonPlugin)
    }
}

#[derive(Resource, Default)]
pub struct GUITextureHandle {
    handle: Handle<Image>,
}
pub struct HiveboticaGUIStartupPlugin;

impl Plugin for HiveboticaGUIStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gui_texture_loader);
    }
}

pub fn gui_texture_loader(
    asset_server: Res<AssetServer>,
    mut gui_texture_handle: ResMut<GUITextureHandle>
) {
    gui_texture_handle.handle = asset_server.load("gui.png");
}
