use bevy::prelude::*;
use std::time::SystemTime;


#[derive(Resource, Default)]
pub struct NTPAdjustment {
    pub system_clock_error: i64,
}

pub fn check_ntp_time(mut ntp_adjustment: ResMut<NTPAdjustment>) {

    // This is just test code for before I add multiplayer. Eventually, this will need to consult a NTP server.
    ntp_adjustment.system_clock_error = 0;

}

// Unix time is the number of milliseconds since 12:00 am UTC on 1/1/70.

pub fn universal_unix_mill_time_is (system_clock_error: i64) -> u128 {

    let system_unix_time: u128;
    let universal_unix_time: u128;
   
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(ut) => {system_unix_time = ut.as_millis();},
        Err(_) => panic!("System time is before Unix epoch."), 
    }

    if system_clock_error < 0 {

    universal_unix_time = system_unix_time.wrapping_sub(((system_clock_error * -1) as u128));

    } else { 

    universal_unix_time = system_unix_time.wrapping_add((system_clock_error as u128));

    }
    
universal_unix_time

}


