use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub (in super) mod gameworld_manager;
pub (in super) mod gamesector_simulator;
pub (in super) mod travelator_simulator;
pub struct HiveboticaSimulationPluginGroup;

impl PluginGroup for HiveboticaSimulationPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(gameworld_manager::HiveboticaGameworldManagerPlugin)
    }
}