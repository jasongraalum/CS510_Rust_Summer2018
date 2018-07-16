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
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {
    // Robot constructor
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x, y, d }
    }

    // Consume robot and create an identical robot but facing 90 degrees to the right
    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    // Consume robot and create an identical robot but facing 90 degrees to the left
    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
        }
    }

    // Consume robot and create an identical robot but moved forward in the direction the
    // original was facing
    pub fn advance(self) -> Self {
        Robot {
            x: match self.d {
                Direction::North => self.x,
                Direction::East => self.x + 1,
                Direction::South => self.x,
                Direction::West => self.x - 1,
            },
            y: match self.d {
                Direction::North => self.y + 1,
                Direction::East => self.y,
                Direction::South => self.y - 1,
                Direction::West => self.y,
            },
            d: self.d,
        }
    }

    // Consume the robot, create a new robot having executed the
    // first command on the command string.
    // Then send the new robot the remaining commands left on the
    // command string
    pub fn instructions(self, instructions: &str) -> Self {
        // If no more instructions, return unmodified robot
        if instructions.is_empty() {
            return self;
        }
        //
        // Take first instruction off the string, operate
        // and then send robot remaining instructions
        //
        match instructions.split_at(1) {
            ("R", i) => self.turn_right().instructions(i),
            ("L", i) => self.turn_left().instructions(i),
            ("A", i) => self.advance().instructions(i),
            _ => self,
        }
    }

    // Return robot x,y position
    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // Return robot direction
    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
