use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub (in super) mod camera;

pub struct HiveboticaGraphicsPluginGroup;

impl PluginGroup for HiveboticaGraphicsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(camera::HiveboticaCameraPlugin)
    }
}