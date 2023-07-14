use bevy::prelude::*;

pub fn generate_base_sector_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut generate_base_sector_map: EventReader<crate::GenerateBaseSectorMap>
) {
    if !generate_base_sector_map.is_empty() {
    
    

       
    }
    generate_base_sector_map.clear();
}
