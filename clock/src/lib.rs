use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {
            minutes: 0,
            hours: ((hours % 24) + 24) % 24,
        };
        clock.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours_add = 
            if minutes + self.minutes >= 0 {
                (minutes + self.minutes) / 60
            } else {
                (minutes - 59) / 60
            };
        let minutes_add = 
            if minutes >= 0 {(self.minutes + minutes) % 60} 
            else {(((self.minutes + minutes) % 60) + 60) % 60};
        Clock {
            hours: ((self.hours + hours_add) % 24 + 24) % 24,
            minutes: minutes_add,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
