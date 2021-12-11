use crate::utils;
use std::collections::HashMap;

pub fn day8() -> (usize, usize) {
    let input: Vec<String> = utils::parse_input("input/day8.txt");
    let parsed_lines: Vec<Vec<&str>> = input
        .iter()
        .map(|line| line.trim().split('|').map(|x| x.trim()).collect())
        .collect();
    let mut part1: usize = 0;
    let mut part2: u64 = 0;
    for line in parsed_lines {
        let patterns: Vec<&str> = line.get(0).unwrap().split(' ').collect();
        let digits: Vec<&str> = line.get(1).unwrap().split(' ').collect();
        let mut raw_bitmaps: [usize; 10] = [0; 10];
        let mut digit_map: HashMap<usize, usize> = HashMap::new();
        let mut reverse_digit_map: HashMap<usize, usize> = HashMap::new();
        for digit in digits.iter() {
            match digit.len() {
                2 | 3 | 4 | 7 => part1 += 1,
                _ => (),
            }
        }

        // Positions are [0,1,2,3,4,5,6] = [top, top-left, top-right, middle, bottom-left, bottom-right, bottom]
        // Digits 1, 4, 7, 8 immediately identifiable.
        // Digit 1 defines positions 2 and 5, but not the order
        // Then 7 defines position 0 uniquely.
        // 4 defines positions 1 and 3 but not the order
        // 3 is identifiable because it's the only 5-segment digit with positions 2 and 5 (shared with 1). It also now defines position 6 uniquely (its the one that's not in common with 1, 4 and 7)
        // 5 is identifiable because it has 3 segments in common with 4, while 2 only has 2 segments in common with 4.
        // We now can identify 1, 2, 3, 4, 5, 7, 8 - need 0, 6, 9
        // 6 does not fully overlap 1
        // 0 does not fully overlap 4
        // And we're done - the remaining digit is 9. We don't need to understand which letter maps to which position.
        // Let's set bits for each letter and then do bit mapping using the logic above to solve for the letters
        for (index, digit) in patterns.iter().enumerate() {
            raw_bitmaps[index] = get_bitmap(digit);
            match digit.len() {
                2 => {
                    digit_map.insert(1, raw_bitmaps[index]);
                    reverse_digit_map.insert(raw_bitmaps[index], 1);
                }
                3 => {
                    digit_map.insert(7, raw_bitmaps[index]);
                    reverse_digit_map.insert(raw_bitmaps[index], 7);
                }
                4 => {
                    digit_map.insert(4, raw_bitmaps[index]);
                    reverse_digit_map.insert(raw_bitmaps[index], 4);
                }
                7 => {
                    digit_map.insert(8, raw_bitmaps[index]);
                    reverse_digit_map.insert(raw_bitmaps[index], 8);
                }
                _ => (),
            }
        }

        // Now deduce the rest of the bitmap values
        for (index, digit) in patterns.iter().enumerate() {
            //Deduce the 5-character digits
            match digit.len() {
                5 => {
                    if !digit_map.contains_key(&3)
                        && (digit_map.get(&1).unwrap() & raw_bitmaps[index]
                            == *digit_map.get(&1).unwrap())
                    {
                        digit_map.insert(3, raw_bitmaps[index]);
                        reverse_digit_map.insert(raw_bitmaps[index], 3);
                    } else if (!digit_map.contains_key(&5)) || (!digit_map.contains_key(&2)) {
                        let mut test_num = digit_map.get(&4).unwrap() & raw_bitmaps[index];
                        let mut count_bits = 0;
                        while test_num > 0 {
                            if test_num & 1 == 1 {
                                count_bits += 1;
                            }
                            test_num >>= 1;
                        }
                        if count_bits == 2 {
                            digit_map.insert(2, raw_bitmaps[index]);
                            reverse_digit_map.insert(raw_bitmaps[index], 2);
                        } else {
                            assert!(count_bits == 3);
                            digit_map.insert(5, raw_bitmaps[index]);
                            reverse_digit_map.insert(raw_bitmaps[index], 5);
                        }
                    }
                }
                6 => {
                    if !digit_map.contains_key(&6)
                        && (digit_map.get(&1).unwrap() & raw_bitmaps[index]
                            != *digit_map.get(&1).unwrap())
                    {
                        digit_map.insert(6, raw_bitmaps[index]);
                        reverse_digit_map.insert(raw_bitmaps[index], 6);
                    } else if !digit_map.contains_key(&0)
                        && (digit_map.get(&4).unwrap() & raw_bitmaps[index]
                            != *digit_map.get(&4).unwrap())
                    {
                        digit_map.insert(0, raw_bitmaps[index]);
                        reverse_digit_map.insert(raw_bitmaps[index], 0);
                    } else if let std::collections::hash_map::Entry::Vacant(e) = digit_map.entry(9) {
                         e.insert(raw_bitmaps[index]);
                         reverse_digit_map.insert(raw_bitmaps[index], 9);
                     }
                }
                _ => (),
            }
        }

        for (index, digit) in digits.iter().enumerate() {
            let digit_value = reverse_digit_map.get(&get_bitmap(digit)).unwrap();
            part2 += *digit_value as u64 * 10_u64.pow((3 - index).try_into().unwrap());
        }
    }

    (part1, part2 as usize)
}

fn get_bitmap(digit: &str) -> usize {
    let mut bitmap: usize = 0;
    for character in digit.chars() {
        match character {
            'a' => bitmap |= 1,
            'b' => bitmap |= 2,
            'c' => bitmap |= 4,
            'd' => bitmap |= 8,
            'e' => bitmap |= 16,
            'f' => bitmap |= 32,
            'g' => bitmap |= 64,
            _ => panic!("Unrecognised character in digit string!"),
        };
    }
    bitmap
}
