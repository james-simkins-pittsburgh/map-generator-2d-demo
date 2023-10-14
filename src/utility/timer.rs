use bevy::prelude::*;
use std::time;

#[derive(Resource, Default)]
pub struct NTPAdjustment {
    pub system_clock_error: i64,
}

fn check_ntp_time(mut ntp_adjustment: ResMut<NTPAdjustment>) {

    // This is just test code for before I add multiplayer. Eventually, this will need to consult a NTP server.
    ntp_adjustment.system_clock_error = 0;

}

// Unix time is the number of milliseconds since 12:00 am UTC on 1/1/70.

fn universal_unix_mill_time_is (system_clock_error: i64) -> u64 {

2

}


