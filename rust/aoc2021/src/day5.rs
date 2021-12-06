use crate::utils;
use regex::Captures;
use regex::Regex;
use std::cmp::*;
use std::collections::HashMap;

pub fn day5() -> (usize, usize) {
    let data: Vec<String> = utils::parse_input("input/day5.txt");
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    let re: Regex = Regex::new(r"(\d+),(\d+)\D+(\d+),(\d+)").unwrap();
    //Values of the vector are the points defining the line (x1, y1, x2, y2)
    //Can't be bothered with a Struct here.
    let mut lines: Vec<(usize, usize, usize, usize)> = vec![];

    for textline in data {
        let caps: Captures = re.captures(&textline).unwrap();
        let line = (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        );
        lines.push(line);
    }

    //Build up the map for horizontal lines only
    for line in lines {
        //Only consider horizontal
        if line.0 == line.2 {
            for i in min(line.1, line.3)..=max(line.1, line.3) {
                let val = match map.get(&(line.0, i)) {
                    //Key present? Increment value by 1
                    Some(x) => x + 1,
                    None => 1,
                };
                map.insert((line.0, i), val);
            }
        } else if line.1 == line.3 {
            for i in min(line.0, line.2)..=max(line.0, line.2) {
                let val = match map.get(&(i, line.1)) {
                    //Key present? Increment value by 1
                    Some(x) => x + 1,
                    None => 1,
                };
                map.insert((i, line.1), val);
            }
        } else {
            //diagonal - this isn't pretty but it's not interesting enough for me to care

            if line.2 > line.0 && line.3 > line.1 {
                let mut j = line.1;
                for i in line.0..=line.2 {
                    let val = match map.get(&(i, j)) {
                        //Key present? Increment value by 1
                        Some(x) => x + 1,
                        None => 1,
                    };
                    map.insert((i, j), val);
                    j += 1;
                }
            } else if line.2 > line.0 && line.3 < line.1 {
                let mut j = line.1;
                for i in line.0..=line.2 {
                    let val = match map.get(&(i, j)) {
                        //Key present? Increment value by 1
                        Some(x) => x + 1,
                        None => 1,
                    };
                    map.insert((i, j), val);
                    j -= 1;
                }
            } else if line.2 < line.0 && line.3 > line.1 {
                let mut j = line.1;
                for i in (line.2..=line.0).rev() {
                    let val = match map.get(&(i, j)) {
                        //Key present? Increment value by 1
                        Some(x) => x + 1,
                        None => 1,
                    };
                    map.insert((i, j), val);
                    j += 1;
                }
            } else {
                assert!(line.2 < line.0 && line.3 < line.1);
                let mut j = line.1;
                for i in (line.2..=line.0).rev() {
                    let val = match map.get(&(i, j)) {
                        //Key present? Increment value by 1
                        Some(x) => x + 1,
                        None => 1,
                    };
                    map.insert((i, j), val);
                    j -= 1;
                }
            }
        }
    }

    map.retain(|_, &mut v| v > 1);
    let part1 = map.len();

    (part1, 2)
}
