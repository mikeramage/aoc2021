use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Octopus<> {
    id: usize,
    energy: usize, 
    flashed_this_turn: bool,
}

impl PartialEq for Octopus {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Octopus {}

impl Hash for Octopus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Octopus {
    pub fn new(id: usize, energy_level: usize) -> Octopus {
        let octopus: Octopus = Octopus {
            id: id,
            energy: energy_level,
            flashed_this_turn: false,
        };
        octopus
    }

    pub fn increment_energy(&mut self) -> bool {
        let mut resulted_in_flash: bool = false;
        if !self.flashed_this_turn {
            self.energy += 1;
            if self.energy > 9 {
                self.energy = 0;
                self.flashed_this_turn = true;
                resulted_in_flash = true;
            }
        }
        resulted_in_flash
    }

    pub fn relax(&mut self) {
        self.flashed_this_turn = false;
    }
}