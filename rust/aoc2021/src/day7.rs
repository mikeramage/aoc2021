use crate::utils;

pub fn day7() -> (usize, usize) {
    let crab_input: Vec<String> = utils::parse_input_by_blank_lines("input/day7.txt");
    assert!(crab_input.len() == 1);
    let crabs: Vec<isize> = crab_input[0]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let part1 = calc_fuel_used(&crabs, move_cost_simple);
    let part2 = calc_fuel_used(&crabs, move_cost_triangle);

    (part1, part2)
}

fn calc_fuel_used(crabs: &[isize], move_function: fn(isize, isize) -> usize) -> usize {
    let max_crab = crabs.iter().max().unwrap();
    let min_crab = crabs.iter().min().unwrap();
    let mut current_min_fuel = 0;
    for i in *min_crab..=*max_crab {
        let fuel = crabs.iter().map(|x| move_function(*x, i)).sum();
        if fuel < current_min_fuel || current_min_fuel == 0 {
            current_min_fuel = fuel;
        }
    }

    current_min_fuel
}

fn move_cost_simple(x: isize, i: isize) -> usize {
    (x - i).abs() as usize
}

fn move_cost_triangle(x: isize, i: isize) -> usize {
    ((x - i).abs() * ((x - i).abs() + 1) / 2) as usize
}
