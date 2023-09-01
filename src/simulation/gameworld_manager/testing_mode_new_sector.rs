use bevy::prelude::*;

pub fn add_sector_if_needed(
    mut svs_reader: EventReader<crate::gui::camera::SwitchVisibleSector>,
    camera_sector_coordinates: Res<crate::gui::camera::CameraSectorCoordinates>,
    mut gns_writer: EventWriter<crate::GenerateNewSector>,
    mut sector_to_be_generated: ResMut<crate::simulation::gameworld_manager::SectorToBeGenerated>,
    mut tile_control: ResMut<crate::graphics::tile_map::TileControlForSectorSwitch>,
    sim_sector_query: Query<&crate::simulation::GamesectorBasics>,
) {
    if !svs_reader.is_empty() {
        
        tile_control.gamesector_generated = false;
        tile_control.gamesector_copied = false;
        tile_control.gamesector_drawn = false;

        for sim_sector in sim_sector_query.iter() {
            if
                sim_sector.sector_coordinates.0 == camera_sector_coordinates.sector_x &&
                sim_sector.sector_coordinates.1 == camera_sector_coordinates.sector_y
            {
                tile_control.gamesector_generated = true;
            }
        }

        if tile_control.gamesector_generated == false {
            gns_writer.send(crate::GenerateNewSector);

            sector_to_be_generated.sector_to_be_generated_list.push((
                camera_sector_coordinates.sector_x,
                camera_sector_coordinates.sector_y,
                crate::simulation::gameworld_manager::InitializationType::Player,
            ));

            
        }

        svs_reader.clear();
    }
}
