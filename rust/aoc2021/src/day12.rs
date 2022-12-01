use crate::utils;
use crate::cave;
use std::collections::HashMap;

pub fn day12() -> (usize, usize) {
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    let input: Vec<String> = utils::parse_input("input/day12_test1.txt");
    let mut caves: HashMap<String, cave::Cave> = HashMap::new();
    for line in input {
        let cave_names: Vec<&str> = line.split('-').collect();
        assert!(cave_names.len() == 2);
        let mut cave_1: &cave::Cave;
        let mut cave_2: &cave::Cave;
        if let Some(cave) = caves.get(cave_names[0]) {
            cave_1 = cave;
        }
        else{
            cave_1 = &cave::Cave::new(String::from(cave_names[0]));
        }

        if let Some(cave) = caves.get(cave_names[1]) {
            cave_2 = cave;
        }
        else{
            cave_2 = &cave::Cave::new(String::from(cave_names[1]));
        }

        cave_1.push_connection(&cave_2);
        cave_2.push_connection(&cave_1);

        caves.entry(cave_names[0].to_string()).or_insert(*cave_1);
        caves.entry(cave_names[1].to_string()).or_insert(*cave_2);
        
    }

    println!("{:?}", caves);

    (part1, part2)
}