use crate::utils;

///Day 1 solution
pub fn day1() -> (usize, usize) {
    let measurements: Vec<i64> = utils::parse_input("input/day1.txt");
    (
        get_count_of_increases(&measurements, 1),
        get_count_of_increases(&measurements, 3),
    )
}

/// Takes a vector of measurements and a window size and for each
/// sliding window of size window_size it counts the number of times
/// the sum of the values in the current window is greater than at the
/// previous window position. The window slides along by 1 index each time.    
fn get_count_of_increases(measurements: &[i64], window_size: usize) -> usize {
    assert!(
        measurements.len() > window_size,
        "Window size is greater than the number of measurements!"
    );

    // Since B + C + D - (A + B + C) = D - A ...
    let count: usize = measurements
        .iter()
        .zip(&measurements[window_size..]) //truncates larger vector
        .map(|(x, y)| y - x)
        .filter(|x| x > &0)
        .count();

    count
}
