use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

use crate::SECTOR_SIZE;

pub(super) mod gameworld_manager;
pub(super) mod gamesector_simulator;
pub(super) mod travelator_simulator;
pub struct HiveboticaSimulationPluginGroup;

impl PluginGroup for HiveboticaSimulationPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(gameworld_manager::HiveboticaGameworldManagerPlugin)
    }
}

#[derive(Copy, Clone)]
pub enum TileType {
    Open,
    Vegetated,
    Elevated,
}

enum SectorBiome {
    Base,
    Plains,
    Desert,
    Frozen,
    Ruins,
}

#[derive(Component)]
struct GamesectorTileMap {
    sector_coordinates: (i32, i32),
    sector_biome: SectorBiome,
    tile_array: [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
}

enum UnitClass {
    Trench,
    Building,
    Mothership,
    SupplyBot,
    GroundCombatBot,
    Munition,
    AirCombatBot,
}

enum UnitType {
    Trench,
    InfantryFactory,
    TankFactory,
    ArtilleryFactory,
    AerialDroneFactory,
    EngineeringFactory,
    SupplyDepot,
    IronMine,
    SiliconMine,
    AluminumMine,
    ArchaeologySite,
    SolorPanel,
    BarbedWire,
    Mine,
    Hedgehog,
    TrainTrack,
    Mothership,
    TrainDrone,
    TruckDrone,
    ScoutBot,
    SpecialBot,
    SniperBot,
    InfantryBot,
    AntitankBot,
    StingerBot,
    LightTankBot,
    HeavyTankBot,
    FlakBot,
    HowitzerBot,
    RocketBot,
    SurfaceToAirBot,
    RadarBot,
    SurveillanceDrone,
    MultipurposeDrone,
    HeavyGroundAttackDrone,
    FighterDrone,
    Bullet,
    SniperBullet,
    InfantryShell,
    InfantryRocket,
    ClusterMunition,
    AntitankMissle,
    StingerMissle,
    LightTankRound,
    HeavyTankround,
    FlakRound,
    ArtilleryRound,
    ArtilleryRocket,
    AirtoAirMissle,
    AirMultipurposeMissile,
    AirtoGroundMissle,
    CompoundGroundMissle,
}

#[derive(Component)]
struct GamesectorUnits {
    // First type is x location and second type is y location.
    // Third type is unit class. Forth type is unit type.
    // Fith type is unit health out of 100.
    // Sixth type is unit energy of of 100.
    // Seventh type is unit supply out of 200.
    // Eighth type is visbility of our 100.
    unit_array: Vec<(u16, u16, UnitClass, UnitType, u8, u8, u8, u8)>,
}

#[derive(Bundle)]
pub struct GamesectorBundle {
    gamesector_tile_map: GamesectorTileMap,
}
