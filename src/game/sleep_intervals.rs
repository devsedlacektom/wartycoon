use std::thread::sleep;
use std::time::Duration;

pub fn game_sleep_second() {
    sleep(Duration::from_millis(1000))
}

pub fn game_sleep_two_seconds() {
    sleep(Duration::from_millis(2000))
}

pub fn game_sleep_half_second() {
    sleep(Duration::from_millis(500))
}

pub fn game_round_sleep() {
    game_sleep_second();
}
