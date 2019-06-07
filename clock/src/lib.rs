use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    nb_minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {nb_minutes: 60 * hours + minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock{
            nb_minutes: self.nb_minutes + minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.nb_minutes)
    }
}