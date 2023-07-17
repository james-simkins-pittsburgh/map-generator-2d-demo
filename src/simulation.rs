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
    Ruin1x1,
    Ruin1x2,
    Ruin1x3,
    Ruin2x2,
    Ruin2x3,
}

enum SectorBiome {
    Base,
    Plains,
    Desert,
    Frozen,
    Mountains,
    Ruins,
}

#[derive(Component)]
struct GamesectorBasics {
    sector_coordinates: (i32, i32),
    sector_biome: SectorBiome,
    tile_array: [[(TileType, u8); SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
}

enum UnitClass {
    Trench,
    AboveGroundFortification,
    Building,
    Mothership,
    SupplyBot,
    EngineeringBot,
    GroundCombatBot,
    Munition,
    AirCombatBot,
}

enum UnitType {
    Trench,
    InfantryFactory,
    TankFactory,
    ArtilleryFactory,
    AerialDroneHangar,
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
    EngineeringBot,
    MineBot,
    ScoutBot,
    SpecialBot,
    SniperBot,
    InfantryBot,
    AntitankBot,
    ManPadBot,
    LightTankBot,
    HeavyTankBot,
    FlakBot,
    HowitzerBot,
    RocketBot,
    SurfaceToAirBot,
    RadarBot,
    SurveillanceDrone,
    MultipurposeAttackDrone,
    GroundAttackDrone,
    AirAttackDrone,
    Bullet,
    SniperBullet,
    InfantryShell,
    ClusterMunition,
    Bomblet,
    BombletMine,
    AntitankMissile,
    ManPadMissile,
    LightTankRound,
    HeavyTankRound,
    FlakRound,
    ArtilleryRound,
    ArtilleryRocket,
    AirtoAirMissle,
    AirtoGroundMissileLight,
    AirtoGroundMissileHeavy,
    AirtoGroundMissileCluster,
}

enum GraphicalVariation {
    Standing,
    Moving,
    Firing,
    Building,
    Demolishing,
    ClimbingDown,
    ClimbingUp,
    CrossingTrench,
    StandingEntrenched,
    MovingEntrenched,
    FiringEntrenched,
    DiggingIn,
    DugInStanding,
    DugInFiring,
    Hiding,
    Hidden,
    HiddenFiring,
    Ascending,
    Exploding,
    ShutDown,
    Disintegrating,
    GivingSupply,
}

enum UnitFaction {
    Industrialist,
    Guardian,
    Rogue,
}

struct UnitAttributes {
    x_location: u16,
    y_location: u16,
    height: u16,
    xy_rotation: u16,
    xz_rotation: u16,
    unit_class: UnitClass,
    unit_type: UnitType,
    graphical_variation: GraphicalVariation,
    unit_faction: UnitFaction,
    unit_allegiance_address: u16,
    unit_hive_address: u16,
    player_unit: bool,
    health_out_of_100: u8,
    energy_units: u8,
    supply_one_units: u8,
    supply_two_units: u8,
    visibility: u8,
    visibletoplayer: bool,
    additional_supply_address: u16,
}

#[derive(Component)]
struct GamesectorUnits {
    unit_array: Vec<UnitAttributes>,
}

#[derive(Bundle)]
pub struct GamesectorBundle {
    gamesector_basics: GamesectorBasics,
    gamesector_units: GamesectorUnits,
}
