use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

use crate::SECTOR_SIZE;

pub(super) mod gameworld_manager;
pub(super) mod gamesector_simulator;
pub(super) mod travelator_simulator;
pub mod testing_mode_simtographics_copier;
pub struct HiveboticaSimulationPluginGroup;

impl PluginGroup for HiveboticaSimulationPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(gameworld_manager::HiveboticaGameworldManagerPlugin)
    }
}

#[derive(Copy, Clone, PartialEq)]
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


#[derive(Copy, Clone)]
pub enum SectorBiome {
    Base,
    Plains,
    Desert,
    Frozen,
    Mountains,
    Ruins,
}

#[derive(Component)]
pub struct GamesectorBasics {
    pub sector_coordinates: (i32, i32),
    pub sector_biome: SectorBiome,
    pub tile_array: [[(TileType, u8); SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
}

pub enum UnitClass {
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

pub enum UnitType {
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
    TrainBot,
    TruckBot,
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
    RogueBotScavenger,
    RogueBotAmbusher,
    RogueBotDrone,
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

pub enum GraphicalVariation {
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

pub enum UnitFaction {
    Industrialist,
    Guardian,
    Rogue,
}

pub struct UnitAttributes {
    pub x_location: u16,
    pub y_location: u16,
    pub height: u16,
    pub xy_rotation: u16,
    pub xz_rotation: u16,
    pub unit_class: UnitClass,
    pub unit_type: UnitType,
    pub graphical_variation: GraphicalVariation,
    pub unit_faction: UnitFaction,
    pub unit_allegiance_address: u16,
    pub unit_hive_address: u16,
    pub player_unit: bool,
    pub health_out_of_100: u8,
    pub energy_in_units: u8,
    pub supply_one_in_units: u8,
    pub supply_two_in_units: u8,
    pub visibility_out_of_100: u8,
    pub visible_to_player: bool,
    pub additional_supply_address: u16,
}

#[derive(Component)]
pub struct GamesectorUnits {
   pub unit_array: Vec<UnitAttributes>,
}

#[derive(Bundle)]
pub struct GamesectorBundle {
    pub gamesector_basics: GamesectorBasics,
    pub gamesector_units: GamesectorUnits,
}
