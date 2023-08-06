use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub(super) mod camera;
pub(super) mod warp_buttons;

pub struct HiveboticaGUIPluginGroup;

impl PluginGroup for HiveboticaGUIPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(camera::HiveboticaCameraPlugin)
    }
}