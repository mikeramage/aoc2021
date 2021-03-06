use crate::position;
use crate::utils;

/// Day 2 solution
pub fn day2() -> (usize, usize) {
    let instructions: Vec<String> = utils::parse_input("input/day2.txt");
    let mut position = position::Position::new();
    let mut position_with_aim = position::Position::new();

    for instruction in instructions {
        let instr_vec: Vec<&str> = instruction.split(' ').collect();
        let command: &str = instr_vec[0];
        let distance: usize = instr_vec[1].parse().unwrap();
        match command {
            "forward" => {
                position.move_forward(distance);
                position_with_aim.move_forward_with_aim(distance);
            }
            "down" => {
                position.move_down(distance);
                position_with_aim.aim_down(distance);
            }
            "up" => {
                position.move_up(distance);
                position_with_aim.aim_up(distance);
            }
            _ => panic!("Aargh, what's this!"),
        }
    }

    (position.area(), position_with_aim.area())
}
