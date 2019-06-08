// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let new_dir = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot::new(self.x, self.y, new_dir)
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let new_dir = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Robot::new(self.x, self.y, new_dir)
    }

    pub fn advance(self) -> Self {
        use Direction::*;
        let (new_x, new_y) = match self.direction {
            North => (self.x, self.y + 1),
            South => (self.x, self.y - 1),
            East => (self.x + 1, self.y),
            West => (self.x - 1, self.y),
        };
        Robot::new(new_x, new_y, self.direction)
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            robot = match instruction {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => unimplemented!("this should never happen"),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
