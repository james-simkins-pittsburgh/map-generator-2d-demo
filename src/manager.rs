// This is the manager module. It tells which systems to run on a given cycle of the Bevy engine.

use bevy::prelude::*;
use crate::utility::timer;

#[derive(Resource, Default)]
pub struct TaskChecklist {
    // This is the unix start time of the turn
    pub turn_start_time: u128,
    // This is the checklist to ensure all 121 updates are applied.
    pub simulation_updates: [[bool; 11]; 11],
    // This indicated whether or not all updates are finished.
    pub all_updates_finished: bool,
    // This is the checklist to ensure all 121 worlds are ticked 20 times both for the first and second parts of each tick.
    pub simulation_ticks: [[[[bool; 11]; 11]; 20]; 2],
    // This indicates whether or not all ticks are finished.
    pub all_ticks_finished: bool,
    // This indicates when player moves have been sent.
    pub player_moves_sent: [[bool; 11]; 11],
    // This indicates when all player moves have been sent.
    pub all_player_moves_sent: bool,
    // This indicates when server updates have been sent.
    pub sector_updates_sent: [[bool; 11]; 11],
    // This indicates when all server updates have been sent.
    pub all_sector_updates_sent: bool,
    // This indicated when new sector_updated have been received.
    pub sector_updates_received: [[bool; 11]; 11],
    // This indicates when all server updates have been received.
    pub all_sector_updates_received: bool,
}

#[derive(Resource, Default)]
pub struct ToDoList {
    pub update_simulation: [[bool; 11]; 11],
    pub step_stimulation: [[[bool; 2]; 11]; 11],
    pub send_player_moves: [[bool; 11]; 11],
    pub send_sector_updates: [[bool; 11]; 11],
}

pub fn set_tasks(
    mut task_checklist: ResMut<TaskChecklist>,
    mut to_do_list: ResMut<ToDoList>,
    ntp_adjustment: Res<crate::utility::timer::NTPAdjustment>
) {
    // This sets the time of the turn to the unix time UTC.

    let unix_cycle_time = timer::universal_unix_mill_time_is(ntp_adjustment.system_clock_error);

    // This sets the delta time of the turn based on the turn start time.

    let mut turn_delta_time: u128;

    if unix_cycle_time > task_checklist.turn_start_time {
        turn_delta_time = unix_cycle_time - task_checklist.turn_start_time;
    } else {
        turn_delta_time = 0;
    }

    // This checks to see if it is time to start a new turn.
    // If it is time to start a new turn it sets the turn time to 2000 more than the previous time.

    if turn_delta_time > 2000 && task_checklist.all_sector_updates_received {

        // This advances the turn start time by 2 seconds (2000 milliseconds).

        task_checklist.turn_start_time = task_checklist.turn_start_time + 2000;

        // This resets delta time based on the new turn start time.

        if unix_cycle_time > task_checklist.turn_start_time {
            turn_delta_time = unix_cycle_time - task_checklist.turn_start_time;
        } else {
            turn_delta_time = 0;
        }

        // This will set all the values for the checklists to false (unchecked).

        for index_1 in 0..11 {
            for index_2 in 0..11 {
                task_checklist.simulation_updates[index_1][index_2] = false;
            }
        }

        task_checklist.all_updates_finished = false;

        for index_1 in 0..11 {
            for index_2 in 0..11 {
                for index_3 in 0..20 {
                    for index_4 in 0..2 {
                        task_checklist.simulation_ticks[index_1][index_2][index_3][index_4] = false;
                    }
                }
            }
        }

        task_checklist.all_ticks_finished = false;

        for index_1 in 0..11 {
            for index_2 in 0..11 {
                task_checklist.player_moves_sent[index_1][index_2] = false;
            }
        }

        task_checklist.all_player_moves_sent = false;

        for index_1 in 0..11 {
            for index_2 in 0..11 {
                task_checklist.sector_updates_sent[index_1][index_2] = false;
            }
        }

        task_checklist.all_sector_updates_sent = false;

        for index_1 in 0..11 {
            for index_2 in 0..11 {
                task_checklist.sector_updates_received[index_1][index_2] = false;
            }
        }

        task_checklist.all_sector_updates_received = false;
    }

    // This part clears the to do list.


    for index_1 in 0..11 {
        for index_2 in 0..11 {
            to_do_list.update_simulation [index_1][index_2] = false;
        }
    }

    for index_1 in 0..2 {
        for index_2 in 0..11 {
            for index_3 in 0..11 {
                to_do_list.step_stimulation [index_1][index_2][index_3] = false;

            }
        }
    }

    
    for index_1 in 0..11 {
        for index_2 in 0..11 {
            to_do_list.send_player_moves [index_1][index_2] = false;
        }
    }

    for index_1 in 0..11 {
        for index_2 in 0..11 {
            to_do_list.send_sector_updates [index_1][index_2] = false;
        }
    }

    // This part checks to see if simulation updates have been applied yet. If not, it schedules them. 
    // The current code is very basic and doesn't take into account potential late updates or other problems.

    if !task_checklist.all_updates_finished {

        for index_1 in 0..11 {

            for index_2 in 0..11 {

                if task_checklist.simulation_updates [index_1][index_2] == false {

                to_do_list.update_simulation [index_1][index_2] = true;
                task_checklist.simulation_updates [index_1][index_2] = true;

                }
            }
        }
    
    task_checklist.all_updates_finished = true;

    }



}
