use std::{thread, time::Duration};

pub fn logging_allarm(message: &str) {
    log::error!("{message}");
    thread::sleep(Duration::from_millis(1000));
}
