use crate::utils;
use crate::octopus;
use std::collections::HashMap;
use std::collections::HashSet;

const MAX_OCTOPI: usize = 10;

pub fn day11() -> (usize, usize) {
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    let mut id: usize = 0;

    let octopi_energies: Vec<Vec<usize>> = utils::parse_input_usizes("input/day11.txt");
    // Map of coordinates to Octopus. Owns the octopi.
    let mut octopi: HashMap<(usize, usize), octopus::Octopus> = HashMap::new();
    
    let mut neighbours: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for (x, octopus_energy_row) in octopi_energies.iter().enumerate() {
        for (y, octopus_energy) in octopus_energy_row.iter().enumerate() {
            let octopus = octopus::Octopus::new(id, *octopus_energy);
            id += 1;
            octopi.insert((x, y), octopus);
            neighbours.insert((x, y), get_neighbours((x, y)));
        }
    }

    loop {
        // Vector of coordinates of Octopi affected by a flash. A coordinate can appear more 
        // than once in the list. 
        let mut affected_octopi: Vec<(usize, usize)> = vec![];
        let mut flashed = 0;
        for (coords, octopus) in octopi.iter_mut() {
            if octopus.increment_energy() {
                flashed += 1;
                if part2 < 100 {
                    part1 += 1;
                }
                for neighbour in neighbours.get(&coords).unwrap() {
                    affected_octopi.push(*neighbour);
                }
            }
        }
        while affected_octopi.len() > 0 {
            let mut new_affected_octopi: Vec<(usize, usize)> = vec![];
            for coords in affected_octopi {
                let octopus = octopi.get_mut(&coords).unwrap();
                if octopus.increment_energy() {
                    flashed += 1;
                    if part2 < 100 {
                        part1 += 1;
                    }
                    for neighbour in neighbours.get(&coords).unwrap() {
                        new_affected_octopi.push(*neighbour);
                    }
                }
            }
            affected_octopi = new_affected_octopi; 
        }

        for i in 0..10 {
            for j in 0..10 {
                let octopus = octopi.get_mut(&(i, j)).unwrap();
                octopus.relax();
            }
        }

        part2 += 1;
        if flashed == 100 {
            break;
        }
    }
    (part1, part2)
}

fn get_neighbours(coords: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut neighbours: HashSet<(usize, usize)> = HashSet::new();
    let x_lower_limit: usize;
    let x_upper_limit: usize;
    let y_lower_limit: usize;
    let y_upper_limit: usize;
    
    //Gotta be a better way than this.
    if coords.0 == 0 {
        x_lower_limit = 0;
    }
    else{
        x_lower_limit = coords.0 - 1;
    }

    if coords.0 == (MAX_OCTOPI - 1) {
        x_upper_limit = MAX_OCTOPI - 1;
    }
    else {
        x_upper_limit = coords.0 + 1;
    }

    if coords.1 == 0 {
        y_lower_limit = 0;
    }
    else{
        y_lower_limit = coords.1 - 1;
    }

    if coords.1 == (MAX_OCTOPI - 1) {
        y_upper_limit = MAX_OCTOPI - 1;
    }
    else {
        y_upper_limit = coords.1 + 1;
    }

    for x in x_lower_limit..=x_upper_limit {
        for y in y_lower_limit..=y_upper_limit {
            if !(x == coords.0 && y == coords.1) {
                neighbours.insert((x, y));
            } 
        } 
    }
    neighbours
}