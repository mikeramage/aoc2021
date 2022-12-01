use crate::utils;
use std::collections::HashMap;

pub fn day10() -> (usize, usize) {
    let mut part1: usize = 0;
    let part2: usize;
    // Combine for part1 and part2. Note for part 2, points use the openers
    // because that's what I'll be popping off the stack.
    let points: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    let input: Vec<Vec<char>> = utils::parse_input_chars("input/day10.txt");
    // This is a stack. We push openers onto it and pop them back off when we find the closer
    // If we find an invalid closer,
    let mut scores: Vec<usize> = vec![];
    for line in &input {
        let mut line_valid = true;
        let mut chunks: Vec<char> = vec![];
        for symbol in line {
            if is_opener(&symbol) {
                // Push the opener onto the stack
                chunks.push(*symbol);
            } else if let Some(x) = chunks.last() {
                // X is the last character in the stack - the one to check this character against.
                if is_valid_closer(x, &symbol) {
                    // Found a match. Pop off the last symbol from the stack - we've found its closer.
                    chunks.pop();
                } else {
                    // Not an opener - we found those above. It must be a closer,and a bad one.
                    // Terminate the inner loop and add up our points.
                    part1 += points.get(&symbol).unwrap();
                    line_valid = false;
                    break;
                }
            } else {
                // Handle bad closer being first symbol in the line or start of a new section of the line
                // following matching all previous openers.
                part1 += points.get(&symbol).unwrap();
                line_valid = false;
                break;
            }
        }

        if line_valid {
            //Can now do part 2 from remaining symbols in chunks.
            let mut score: usize = 0;
            while chunks.len() > 0 {
                score = 5 * score + *points.get(&chunks.pop().unwrap()).unwrap();
            }
            scores.push(score);
        }
    }

    scores.sort();
    let median_index = (scores.len() - 1) / 2;
    part2 = scores[median_index];

    (part1, part2)
}

fn is_valid_closer(current_symbol: &char, symbol_under_test: &char) -> bool {
    match symbol_under_test {
        ')' => *current_symbol == '(',
        ']' => *current_symbol == '[',
        '}' => *current_symbol == '{',
        '>' => *current_symbol == '<',
        _ => false,
    }
}

fn is_opener(symbol_under_test: &char) -> bool {
    match symbol_under_test {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}
