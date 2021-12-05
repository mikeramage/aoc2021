use crate::utils;
use crate::bingo;
use std::collections::HashSet;

pub fn day4() -> (usize, usize) {
    let data: Vec<String> =  utils::parse_input_by_blank_lines("input/day4.txt");
    let numbers: &String = &data[0];
    let boards_input: &[String] = &data[1..];
    let mut boards: Vec<bingo::Board> = vec![];
    let mut part2 = 0;
    let mut part1 = 0;
    for board in boards_input {
        boards.push(bingo::Board::new(board));
    }
    let boards_len = boards.len();

    let mut boards_that_have_won = HashSet::new();
    'outer: for number in numbers.split(',') {
        for (index, board) in boards.iter_mut().enumerate() {
            let (win, sum) = board.handle_number(number);
            if let bingo::BingoResult::Win = win {
                if part1 == 0 {
                    part1 = sum;
                }
                if !boards_that_have_won.contains(&index) {
                    //This will keep overwriting until the last board. 
                    //Works even if not all boards have won
                    part2 = sum;
                }
                boards_that_have_won.insert(index);
            }
            if boards_that_have_won.len() == boards_len {
                //Short circuits to avoid handling any more numnbers needlessly.
                break 'outer;
            }    
        }
    }

    (part1, part2)
}