#[derive(Debug)]
pub struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn move_forward(&mut self, distance: usize) {
        self.horizontal += distance;
    }

    //We allow negative depths for now - this is a flying submarine!
    pub fn move_up(&mut self, distance: usize) {
        self.depth -= distance;
    }

    pub fn move_down(&mut self, distance: usize) {
        self.depth += distance;
    }

    pub fn move_forward_with_aim(&mut self, distance: usize) {
        self.horizontal += distance;
        self.depth += distance * self.aim;
    }

    //We allow negative depths for now - this is a flying submarine!
    pub fn aim_up(&mut self, distance: usize) {
        self.aim -= distance;
    }

    pub fn aim_down(&mut self, distance: usize) {
        self.aim += distance;
    }

    //Probably doesn't make much sense in a "position" object, but
    //it's the answer!
    pub fn area(&self) -> usize {
        self.horizontal * self.depth
    }
}
