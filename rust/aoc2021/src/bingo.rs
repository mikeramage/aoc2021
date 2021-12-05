use std::collections::HashMap;

const DIM: usize = 5;

#[derive(Debug)]
pub struct Board {
    unmarked: HashMap<String, (usize, usize)>,
    marked: HashMap<String, (usize, usize)>,
    row_marked_count: [usize; DIM],
    col_marked_count: [usize; DIM],
}

pub enum BingoResult {
    NoWin,
    Win,
}

impl Board {
    pub fn new(board_input: &str) -> Board {
        // Initialize a Board
        let mut board: Board = Board {
            row_marked_count: [0; DIM],
            col_marked_count: [0; DIM],
            unmarked: HashMap::new(),
            marked: HashMap::new(),
        };

        // Read in rows and columns of board into string vecs.
        // For now the values (i.e. numbers) are strings - we'll do the conversion
        // at the end when we need to.
        let rows: Vec<&str> = board_input.split('\n').collect();
        for (row_index, row) in rows.iter().enumerate() {
            let col_vals: Vec<&str> = row.trim().split_whitespace().collect();
            for (col_index, val) in col_vals.iter().enumerate() {
                board
                    .unmarked
                    .insert(val.to_string(), (row_index, col_index));
            }
        }

        board
    }

    /// If number is in the keys of the unmarked map, remove it from the map and add it to the marked map.
    /// Then increment the appropriate row_marked_count and col_marked_count array entries.
    /// If either of these counts is zero - then bingo (return true)! Otherwise false
    pub fn handle_number(&mut self, number: &str) -> (BingoResult, usize) {
        let mut num: usize = 0;
        let mut sum: usize = 0;
        let mut result: BingoResult = BingoResult::NoWin;
        if let Some((key, val)) = self.unmarked.remove_entry(number) {
            self.marked.insert(key, val);
            self.row_marked_count[val.0] += 1;
            self.col_marked_count[val.1] += 1;
            if self.row_marked_count[val.0] == DIM || self.col_marked_count[val.1] == DIM {
                num = number.parse().unwrap();
                sum = self
                    .unmarked
                    .keys()
                    .map(|k| k.parse::<usize>().unwrap())
                    .sum();
                result = BingoResult::Win;
            }
        }
        (result, sum * num)
    }
}
