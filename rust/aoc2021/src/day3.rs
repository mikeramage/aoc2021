use crate::utils;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

pub fn day3() -> (usize, usize) {
    let report: Vec<String> = utils::parse_input("input/day3.txt");
    let report_as_uints: Vec<usize> = report
        .iter()
        .map(|x| utils::string_binary_to_usize(x.to_string()))
        .collect();
    let mut gamma = 0;
    let mut epsilon = 0;
    let num_digits = report[0].len();
    for i in 0..num_digits {
        match cmp_bit_val_in_position(&report_as_uints, i) {
            Less => epsilon += 1 << i,
            Greater => gamma += 1 << i,
            _ => panic!("What the bejingles?"),
        }
    }

    let report_as_uints_clone = report_as_uints.clone();
    let oxy = get_oxy_or_scrub(report_as_uints_clone, num_digits, oxy_criteria);
    //Might as well much the original report_as_uints since we don't need it anymore.
    let scrub = get_oxy_or_scrub(report_as_uints, num_digits, scrub_criteria);

    (gamma * epsilon, oxy * scrub)
}

fn cmp_bit_val_in_position(vals: &[usize], index: usize) -> Ordering {
    let mask = 1 << index;
    let mut count_ones = 0;
    for x in vals {
        if x & mask != 0 {
            count_ones += 1;
        }
    }

    let count_zeros = vals.len() - count_ones;
    count_ones.cmp(&count_zeros)
}

fn get_oxy_or_scrub(
    mut vals: Vec<usize>,
    num_digits: usize,
    func: fn(usize, usize, Ordering) -> bool,
) -> usize {
    for index in (0..num_digits).rev() {
        let mask: usize = 1 << index;
        let val_cmp = cmp_bit_val_in_position(&vals, index);
        vals.retain(|x| func(*x, mask, val_cmp));
        if vals.len() <= 1 {
            break;
        }
    }
    assert!(vals.len() == 1);
    vals[0]
}

fn oxy_criteria(value: usize, mask: usize, val_cmp: Ordering) -> bool {
    (value & mask != 0 && (val_cmp == Greater || val_cmp == Equal))
        || (value & mask == 0 && val_cmp == Less)
}

fn scrub_criteria(value: usize, mask: usize, val_cmp: Ordering) -> bool {
    (value & mask != 0 && (val_cmp == Less))
        || (value & mask == 0 && (val_cmp == Greater || val_cmp == Equal))
}
