// This is the manager module. It tells which systems to run on a given cycle of the Bevy engine.

use bevy::prelude::*;
use crate::utility::timer;

#[derive(Resource, Default)]
pub struct TaskChecklist {

    // This is the checklist to ensure all 121 updates are applied.
    pub simulation_updates: [[bool; 11]; 11],
    // This indicated whether or not all updates are finished.
    pub all_updates_finished: bool,
    // This is the checklist to ensure al 121 worlds are ticked 20 times.
    pub simulation_ticks: [[[[bool; 2]; 11]; 11]; 20],
    // This indicates whether or not all ticks are finished.
    pub all_ticks_finished: bool,
    // This indicates when player moves have been sent.
    pub player_moves_sent: [[bool; 11]; 11],
    // This indicates when all player moves have been sent. 
    pub all_player_moves_sent: bool,
    // This indicated when server updates have been sent.
    pub sector_updates_sent: [[bool; 11]; 11],
    // This indicated when all server updates have been sent.
    pub all_sector_updates_sent: bool,

}

#[derive(Resource, Default)]
pub struct ToDoList {

    pub update_simulation: [[bool; 11]; 11],
    pub step_stimulation: [[[bool; 2]; 11]; 11],
    pub send_player_moves: [[bool; 11]; 11],
    pub send_sector_updates: [[bool; 11]; 11], 

}

fn set_tasks(mut task_checklist: ResMut<TaskChecklist>, mut to_do_list: ResMut<ToDoList>, ntp_adjustment: Res<crate::utility::timer::NTPAdjustment>) {

let unix_time = timer::universal_unix_mill_time_is(ntp_adjustment.system_clock_error);

}
