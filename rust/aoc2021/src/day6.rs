use crate::utils;
use std::collections::HashMap;

pub fn day6() -> (usize, usize) {
    let fish_in: Vec<String> = utils::parse_input_by_blank_lines("input/day6.txt");
    assert!(fish_in.len() == 1);
    let mut fish: Vec<usize> = fish_in[0]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let orig_fish = fish.clone();

    //doesn't scale but quick enough for part1.
    for _i in 1..=80 {
        let num_zeros: usize = fish.iter().filter(|&x| *x == 0).count();
        fish = fish
            .into_iter()
            .map(|x| if x == 0 { 6 } else { x - 1 })
            .collect();
        fish.append(&mut vec![8; num_zeros]);
    }

    let part1 = fish.len();

    //Let's use a hashmap for vals 1 to 8 - keys are days left.
    let mut fishmap: HashMap<usize, usize> = HashMap::new();
    for fish in orig_fish {
        let val = match fishmap.get(&fish) {
            //Key present? Increment value by 1
            Some(x) => x + 1,
            None => 1,
        };
        fishmap.insert(fish, val);
    }

    for _i in 1..=256 {
        let num_zeros = match fishmap.get(&0) {
            Some(x) => *x,
            None => 0,
        };
        for j in 1..=8 {
            let val: usize = match fishmap.get(&j) {
                Some(x) => *x,
                None => 0,
            };
            fishmap.insert(j - 1, val);
        }
        let num_sixes = match fishmap.get(&6) {
            Some(x) => *x,
            None => 0,
        };
        fishmap.insert(6, num_zeros + num_sixes);
        fishmap.insert(8, num_zeros);
    }
    let vec_part2: Vec<usize> = fishmap.into_values().collect();
    let part2 = vec_part2.iter().sum();

    (part1, part2)
}
