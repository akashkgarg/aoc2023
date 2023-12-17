use crate::common::get_lines;
use std::collections::HashSet;

// do the dumb thing to move the rocks.
fn tilt_north(map: &mut Vec<Vec<char>>) -> bool {
    
    let mut moved = false;
    for i in 1..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' && map[i-1][j] == '.' {
                map[i-1][j] = 'O';
                map[i][j] = '.';
                moved = true;
            }
        }
    }
    moved
}

fn tilt_south(map: &mut Vec<Vec<char>>) -> bool {
    
    let mut moved = false;
    for i in (0..=map.len()-2).rev() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' && map[i+1][j] == '.' {
                map[i+1][j] = 'O';
                map[i][j] = '.';
                moved = true;
            }
        }
    }
    moved
}

fn tilt_east(map: &mut Vec<Vec<char>>) -> bool {
    let mut moved = false;
    for i in 0..map.len() {
        for j in (0..=map[i].len()-2).rev() {
            if map[i][j] == 'O' && map[i][j+1] == '.' {
                map[i][j+1] = 'O';
                map[i][j] = '.';
                moved = true;
            }
        }
    }
    moved
}

fn tilt_west(map: &mut Vec<Vec<char>>) -> bool {
    let mut moved = false;
    for i in 0..map.len() {
        for j in 1..map[i].len() {
            if map[i][j] == 'O' && map[i][j-1] == '.' {
                map[i][j-1] = 'O';
                map[i][j] = '.';
                moved = true;
            }
        }
    }
    moved
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn compute_load(map: &Vec<Vec<char>>) -> usize {
    // count rocks for each row
    let mut sum = 0;
    for (row_idx, row) in map.iter().enumerate() {
        let mut rocks = 0;
        for c in row {
            if *c == 'O' {
                rocks += 1;
            }
        }
        let multiplier = map.len() - row_idx;
        sum += rocks * multiplier;
    }

    sum
}

fn cycle(map: &mut Vec<Vec<char>>) {
    while tilt_north(map) { }
    while tilt_west(map) { }
    while tilt_south(map) { }
    while tilt_east(map) { }
}

fn part1(filename: &str) -> usize { 
    let lines = get_lines(filename);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    while tilt_north(&mut map) { }

    compute_load(&map)
}

fn part2(filename: &str) -> usize { 
    let lines = get_lines(filename);

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    let mut maps: Vec<Vec<Vec<char>>> = Vec::new();

    cycle(&mut map);
    let mut iter = 1;
    while maps.iter().any(|m| m == &map) == false {
        maps.push(map.clone());
        cycle(&mut map);
        iter += 1;
        println!("iter: {}", iter);
    }
    let repeating_cycle_start = maps.iter().position(|m| m == &map).unwrap() + 1;
    let repeats_every = iter - repeating_cycle_start;
    let rem = (1000000000 - repeating_cycle_start) % repeats_every;
    println!("repeating cycle start: {}, repeats every {}", repeating_cycle_start, repeats_every);
    println!("need {} more iterations", rem);
    for i in 0..rem {
        cycle(&mut map);
    }
    compute_load(&map)
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day14_sample.txt"), 136);
        assert_eq!(part1("files/day14_input.txt"), 107430);
        assert_eq!(part2("files/day14_sample.txt"), 64);
        assert_eq!(part2("files/day14_input.txt"), 96317);
    }
}