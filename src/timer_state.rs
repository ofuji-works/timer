use std::{
    task::Waker,
    time::{Duration, SystemTime},
};

pub struct TimerState {
    pub time: Duration,
    pub start_time: SystemTime,
    pub pause_time: Option<SystemTime>,
    pub completed: bool,
    pub waker: Option<Waker>,
}

impl TimerState {
    pub fn new(time: Duration, start_time: SystemTime) -> TimerState {
        TimerState {
            time,
            start_time,
            pause_time: None,
            completed: false,
            waker: None,
        }
    }
}

pub fn get_current_time(time: Duration, start_time: SystemTime) -> Duration {
    time - start_time.elapsed().unwrap()
}
