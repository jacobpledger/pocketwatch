use std::time::{SystemTime, Duration};

pub trait Timer {
    fn get_time(&self) -> Duration;
    fn start(&mut self);
    fn stop(&mut self);
    fn set_start(&mut self, to: SystemTime);
    fn set_end(&mut self, to: SystemTime);
}


pub struct LocalTimer {
    start: Option<SystemTime>,
    end: Option<SystemTime>
}

impl Timer for LocalTimer {

    fn get_time(&self) -> Duration {
        if self.end.is_none() {
            if self.start.is_none() {
                return Duration::new(0, 0)
            }
            else {
                self.start.unwrap_or(SystemTime::now()).elapsed().unwrap_or(Duration::new(0,0))
            }
        }
        else {
            return Duration::new(0, 0);
        }
    }

    fn set_start(&mut self, time: SystemTime) {
        self.start = Some(time);
    }

    fn set_end(&mut self, time: SystemTime) {
        self.end = Some(time);
    }

    fn start(&mut self) {
        self.set_start(SystemTime::now());
    }

    fn stop(&mut self) {
        self.set_end(SystemTime::now());
    }

}

impl LocalTimer {
    pub fn new() -> LocalTimer {
        LocalTimer {
            start: None,
            end: None
        }
    }
}