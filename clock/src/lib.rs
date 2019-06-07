use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    nb_minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut increment = 0;
        if  60 * hours + minutes < 0{
            while 60 * hours + minutes + 1440 * increment < 0 {
                increment += 1;
            }
        }
        if 60 * hours + minutes >= 1440 {
            while 60 * hours + minutes + 1440 * increment >= 1440 {
                increment -= 1;
            }
        }
        Clock {nb_minutes: 60 * hours + minutes + 1440 * increment}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0,self.nb_minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.nb_minutes / 60) % 24;
        let minutes = self.nb_minutes % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}