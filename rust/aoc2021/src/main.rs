use std::fs;

fn main() {
    day1()
}

/// Takes a file containing a list of integers, one per line, and returns the 
/// list of integers as a Vec<i32>
fn parse_input_as_int_vec(input_file: &str) -> Vec<i32> {
    let input = fs::read_to_string(input_file)
        .expect("Oh dear, couldn't read file!");
    let mut vector: Vec<i32> = vec![];
    for line in input.lines() {
        vector.push(line.parse::<i32>().expect("Failed to parse line!"));
    }
    vector
}

///Day 1 solution
fn day1() {
   let measurements = parse_input_as_int_vec("day1.txt");
   println!("Part1 - number of measurements greater than previous: {}", 
            get_count_of_increases(&measurements, 1));
   println!("Part2 - number of 3-value window sums greater than previous: {}", 
            get_count_of_increases(&measurements, 3));
}

/// Takes a vector of measurements and a window size and for each 
/// sliding window of size window_size it counts the number of times 
/// the sum of the values in the current window is greater than at the
/// previous window position. The window slides along by 1 index each time.    
fn get_count_of_increases(measurements: &Vec<i32>, 
                          window_size: usize) -> u32 {
    let mut count = 0;
    let mut previous_value = 0;
    assert!(measurements.len() > window_size, 
        "Window size is greater than the number of measurements!");
    for i in 0..window_size {
        previous_value += measurements[i];
    }

    for i in window_size..measurements.len() {
        let mut current_value = 0;
        for j in 0..window_size{
             current_value += measurements[i-j];
        }

        if current_value > previous_value{
            count += 1;
        }

        previous_value = current_value;
    }

    count
}