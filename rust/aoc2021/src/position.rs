#[derive(Debug)]
pub struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn move_forward(&mut self, distance: isize) {
        self.horizontal += distance;
    }

    //We allow negative depths for now - this is a flying submarine!
    pub fn move_up(&mut self, distance: isize) {
        self.depth -= distance;
    }

    pub fn move_down(&mut self, distance: isize) {
        self.depth += distance;
    }

    pub fn move_forward_with_aim(&mut self, distance: isize) {
        self.horizontal += distance;
        self.depth += distance * self.aim;
    }

    //We allow negative depths for now - this is a flying submarine!
    pub fn aim_up(&mut self, distance: isize) {
        self.aim -= distance;
    }

    pub fn aim_down(&mut self, distance: isize) {
        self.aim += distance;
    }

    //Probably doesn't make much sense in a "position" object, but
    //it's the answer!
    pub fn area(&self) -> isize {
        self.horizontal * self.depth
    }
}
