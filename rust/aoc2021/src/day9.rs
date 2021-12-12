use crate::utils;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day9() -> (usize, usize) {
    let mut part2: usize = 0;
    let input: Vec<Vec<usize>> = utils::parse_input_usizes("input/day9.txt");

    let mut low_points: HashSet<(usize, usize)> = HashSet::new();
    let mut risk_level: usize = 0;
    //Determine the low points
    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if is_low_point(x, y, &input) {
                low_points.insert((x, y));
                risk_level += input[x][y] + 1
            }
        }
    }

    // Create a hashmap to store lowpoint coordinate to basin size
    let mut basin_sizes: HashMap<(usize, usize), usize> = HashMap::new();

    // Create a set to store coords of expanded locations. This is to track
    // That we've already explored this location and don't need to consider
    // it again. We could do it on a per-basin basis given the formulation
    // of the problem, but more generally a basin could cover multiple nodes
    // so we just make this global.
    let mut explored_nodes: HashSet<(usize, usize)> = HashSet::new();

    // Iterate over the low points. Expanding each point will yield a "frontier"
    // of new unexplored nodes. Recurse over each node in the frontier till we've
    // exhausted all the nodes - either hit 9s or nodes in the new expansion have
    // already been explored.
    for low_point in low_points {
        let mut frontier: HashSet<(usize, usize)> = HashSet::new();
        frontier.insert(low_point);
        basin_sizes.insert(low_point, 1);
        explored_nodes.insert(low_point);
        while !frontier.is_empty() {
            let mut new_frontier: HashSet<(usize, usize)> = HashSet::new();
            for node in frontier {
                for new_node in expand_node(&node, &input) {
                    if !explored_nodes.contains(&new_node) && input[new_node.0][new_node.1] != 9 {
                        new_frontier.insert(new_node);
                        basin_sizes.insert(low_point, basin_sizes.get(&low_point).unwrap() + 1);
                    }
                    explored_nodes.insert(new_node);
                }
            }
            frontier = new_frontier;
        }
    }

    let mut basin_sizes_vec: Vec<&usize> = basin_sizes.values().collect::<Vec<&usize>>();
    basin_sizes_vec.sort();
    let part_2 = &basin_sizes_vec[basin_sizes_vec.len() - 3..basin_sizes_vec.len()];
    println!("{:?}", part_2);

    (risk_level, part2)
}

fn is_low_point(x: usize, y: usize, input: &Vec<Vec<usize>>) -> bool {
    if x == 0 {
        if y == 0 {
            if input[x + 1][y] > input[x][y] && input[x][y + 1] > input[x][y] {
                return true;
            }
        } else if y == input[x].len() - 1 {
            if input[x + 1][y] > input[x][y] && input[x][y - 1] > input[x][y] {
                return true;
            }
        } else {
            if input[x][y - 1] > input[x][y]
                && input[x][y + 1] > input[x][y]
                && input[x + 1][y] > input[x][y]
            {
                return true;
            }
        }
    } else if x == input.len() - 1 {
        if y == 0 {
            if input[x - 1][y] > input[x][y] && input[x][y + 1] > input[x][y] {
                return true;
            }
        } else if y == input[x].len() - 1 {
            if input[x - 1][y] > input[x][y] && input[x][y - 1] > input[x][y] {
                return true;
            }
        } else {
            if input[x][y - 1] > input[x][y]
                && input[x][y + 1] > input[x][y]
                && input[x - 1][y] > input[x][y]
            {
                return true;
            }
        }
    } else {
        if y == 0 {
            if input[x - 1][y] > input[x][y]
                && input[x][y + 1] > input[x][y]
                && input[x + 1][y] > input[x][y]
            {
                return true;
            }
        } else if y == input[x].len() - 1 {
            if input[x - 1][y] > input[x][y]
                && input[x][y - 1] > input[x][y]
                && input[x + 1][y] > input[x][y]
            {
                return true;
            }
        } else {
            if input[x][y - 1] > input[x][y]
                && input[x][y + 1] > input[x][y]
                && input[x - 1][y] > input[x][y]
                && input[x + 1][y] > input[x][y]
            {
                return true;
            }
        }
    }
    return false;
}

fn expand_node(node: &(usize, usize), input: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut return_vec: Vec<(usize, usize)> = vec![];
    let x = node.0;
    let y = node.1;
    if x == 0 {
        if y == 0 {
            return_vec.push((x, y + 1));
            return_vec.push((x + 1, y));
        } else if y == input[0].len() - 1 {
            return_vec.push((x, y - 1));
            return_vec.push((x + 1, y));
        } else {
            return_vec.push((x, y + 1));
            return_vec.push((x, y - 1));
            return_vec.push((x + 1, y));
        }
    } else if x == input.len() - 1 {
        if y == 0 {
            return_vec.push((x, y + 1));
            return_vec.push((x - 1, y));
        } else if y == input[0].len() - 1 {
            return_vec.push((x, y - 1));
            return_vec.push((x - 1, y));
        } else {
            return_vec.push((x, y + 1));
            return_vec.push((x, y - 1));
            return_vec.push((x - 1, y));
        }
    } else {
        if y == 0 {
            return_vec.push((x, y + 1));
            return_vec.push((x - 1, y));
            return_vec.push((x + 1, y));
        } else if y == input[0].len() - 1 {
            return_vec.push((x, y - 1));
            return_vec.push((x - 1, y));
            return_vec.push((x + 1, y));
        } else {
            return_vec.push((x, y + 1));
            return_vec.push((x, y - 1));
            return_vec.push((x - 1, y));
            return_vec.push((x + 1, y));
        }
    }
    return_vec
}
