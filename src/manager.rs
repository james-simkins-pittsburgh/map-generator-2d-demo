// This is the manager module. It tells which systems to run on a given cycle of the Bevy engine.

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TaskChecklist {
    pub simulation_ticks: [[[bool; 20]; 11]; 11],
    pub all_ticks_finished: bool,
}

#[derive(Resource, Default)]
pub struct ToDoList {
    pub step_stimulation: [[bool; 11]; 11],
    pub all_ticks_finished: bool,

}

fn set_tasks(mut task_checklist: ResMut<TaskChecklist>, mut to_do_list: ResMut<ToDoList>) {




}
