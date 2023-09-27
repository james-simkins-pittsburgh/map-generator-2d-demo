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
    RuinTopLeft,
    RuinTopRight,
    RuinBottomLeft,
    RuinBottomRight,
    RuinRightSide,
    RuinLeftSide,
    RuinTopSide,
    RuinBottomSide
}


#[derive(Copy, Clone, PartialEq)]
pub enum SectorBiome {
    Plains,
    Desert,
    Tundra,
    Alpine,
}


#[derive(Copy, Clone, PartialEq)]
pub enum SectorBaseType {

    Industrialist,
    Guardian,
    Wild,
    
}

#[derive(Component)]
pub struct GamesectorBasics {
    pub sector_coordinates: (i32, i32),
    pub sector_biome: SectorBiome,
    pub tile_array: [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    pub sector_base_type: SectorBaseType,
}


#[derive(Copy, Clone, PartialEq)]
pub enum UnitClass {
    Trench,
    Track,
    Depot,
    Factory,
    Mine,
    Mothership,
    TrainBot,
    TruckBot,
    EngineeringBot,
    GroundCombatBot,
    AirCombatBot,
    Munition,
    Empty,
}



impl Default for UnitClass {
    fn default() -> Self { UnitClass::Empty }
}




#[derive(Copy, Clone, PartialEq)]
pub enum UnitType {
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
    SolarPanel,
    BarbedWire,
    Hedgehog,
    TrainTrack,
    Mothership,
    TrainBot,
    TruckBot,
    EngineeringBot,
    MinelayerBot,
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
    Landmine,
    AntitankMissile,
    ManPadMissile,
    LightTankRound,
    HeavyTankRound,
    FlakRound,
    ArtilleryRound,
    ArtilleryRocket,
    AirtoAirMissile,
    AirtoGroundMissileLight,
    AirtoGroundMissileHeavy,
    AirtoGroundMissileCluster,
    Empty
}


impl Default for UnitType {
    fn default() -> Self { UnitType::Empty }
}



#[derive(Copy, Clone, PartialEq)]
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
    Descending,
    Exploding,
    ShutDown,
    Disintegrating,
    GivingSupply,
    Empty
}

impl Default for GraphicalVariation {
    fn default() -> Self { GraphicalVariation::Empty }
}

#[derive(Copy, Clone, PartialEq)]

pub enum UnitFaction {
    Industrialist,
    Guardian,
    Rogue,
    Empty
}


impl Default for UnitFaction {
    fn default() -> Self { UnitFaction::Empty }
}



#[derive(Copy, Clone, PartialEq)]

pub enum UnitInstructionType {
    MoveTo,
    FireAt,
    Build,
    Repair,
    Advance,
    Sneak,
    Retreat,
    FireAtWill,
    HoldFire,
    DigIn,
    Hide,
    Produce,
    Mine,
    Wait,
}


impl Default for UnitInstructionType{
    fn default() -> Self { UnitInstructionType::Wait}
}





#[derive(Copy, Clone, Default)]
pub struct UnitAttributes {
    pub x_location: u16,
    pub y_location: u16,
    pub altitude: u16,
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

    // First two u16s are the x and y of the target. 
    // The final u8 is the duration of the action in seconds.

    pub instructions: [(UnitInstructionType, u16, u16, u8); 6]

}


#[derive(Component)]
pub struct GamesectorUnits {
   pub unit_vec: Vec<UnitAttributes>,
}



#[derive(Copy, Clone, PartialEq)]

pub enum NewInstructionType {

    Replacement, 
    Addition,
    Override,
    Immediate, 

}

#[derive(Component)]
pub struct InputSimulationUpdates {

    arriving_unit_vec: Vec<UnitAttributes>,
    new_instructions_vec: Vec <(NewInstructionType, (UnitInstructionType, u16, u16, u8))>,

}

#[derive(Component)]
pub struct OutputToTravelator {

    // The address number corresponds to the travelators starting from top left going clockwise.

    departing_units: [(bool, UnitAttributes); 24],
}




#[derive(Bundle)]
pub struct GamesectorBundle {
    pub gamesector_basics: GamesectorBasics,
    pub gamesector_units: GamesectorUnits,
    pub input_simulation_updates: InputSimulationUpdates,
    pub output_to_travelator: OutputToTravelator,
}
