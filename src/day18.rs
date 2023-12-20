use crate::common::get_lines;
use std::collections::HashSet;

type Position = (isize, isize);

// use the shoelace formula to compute the area of a polygon.
// https://en.wikipedia.org/wiki/Shoelace_formula
fn polygon_area(pts: &Vec<Position>, perimeter: isize) -> isize {
    let mut area = 0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        area += (pts[i].1 + pts[j].1) * (pts[i].0 - pts[j].0);
    }
    // Pick's theorem states that A = i + b/2 - 1, where b is the boundary.
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // We want to include the boundary in our area: 
    // A + b = (i + b/2 - 1) + b/2 + 1
    //              ^           ^   
    //         from shoelace   add boundary
    area.abs() / 2 + perimeter / 2 + 1
}

fn part1(filename: &str) -> isize {
    let lines = get_lines(filename);
    let mut curr_pos: Position = (0, 0);
    let mut pts: Vec<Position> = Vec::new();
    pts.push(curr_pos);
    let mut total_len = 0;
    for line in lines {
        let mut tokens = line.split_whitespace();
        let dir = tokens.next().unwrap();
        let len = tokens.next().unwrap().parse::<isize>().unwrap();

        match dir { 
            "R" => curr_pos.0 += len,
            "L" => curr_pos.0 -= len,
            "U" => curr_pos.1 += len,
            "D" => curr_pos.1 -= len,
            _ => panic!("invalid direction"),
        }
        total_len += len;
        pts.push(curr_pos);
    }

    polygon_area(&pts, total_len)
}

fn part2(filename: &str) -> isize {
    let lines = get_lines(filename);
    let mut curr_pos: Position = (0, 0);
    let mut pts: Vec<Position> = Vec::new();
    pts.push(curr_pos);
    let mut total_len = 0;
    for line in lines {
        let mut tokens = line.split_whitespace();
        let _dir = tokens.next().unwrap();
        let _len = tokens.next().unwrap().parse::<isize>().unwrap();
        let hex = tokens.next().unwrap();

        let hex_len = isize::from_str_radix(&hex[2..7], 16).unwrap();
        let hex_dir = hex.chars().nth(7).unwrap();
        match hex_dir { 
            '0' => curr_pos.0 += hex_len,
            '1' => curr_pos.1 -= hex_len,
            '2' => curr_pos.0 -= hex_len,
            '3' => curr_pos.1 += hex_len,
            _ => panic!("invalid direction"),
        }
        pts.push(curr_pos);
        total_len += hex_len;
    }
    polygon_area(&pts, total_len)
}

// the dumb way.
fn part1_floodfill(filename: &str) -> usize {
    let lines = get_lines(filename);
    let mut curr_pos: Position = (0, 0);
    let mut marked_pos: HashSet<Position> = HashSet::new();

    // mark starting
    marked_pos.insert(curr_pos);

    for line in lines {
        let mut tokens = line.split_whitespace();
        let dir = tokens.next().unwrap();
        let len = tokens.next().unwrap().parse::<isize>().unwrap();

        match dir { 
            "R" => {
                for i in 1..=len {
                    marked_pos.insert((curr_pos.0, curr_pos.1 + i));
                }
                curr_pos.1 += len;
            },
            "L" => {
                for i in 1..=len {
                    marked_pos.insert((curr_pos.0, curr_pos.1 - i));
                }
                curr_pos.1 -= len;
            },
            "U" => {
                for i in 1..=len {
                    marked_pos.insert((curr_pos.0 - i, curr_pos.1));
                }
                curr_pos.0 -= len;
            },
            "D" => {
                for i in 1..=len {
                    marked_pos.insert((curr_pos.0 + i, curr_pos.1));
                }
                curr_pos.0 += len;
            },
            _ => panic!("invalid direction"),
        }
    }

    // find extents of the marked positions.
    let mut min_r = isize::MAX;
    let mut max_r = isize::MIN;
    let mut min_c = isize::MAX;
    let mut max_c = isize::MIN;
    for pos in marked_pos.iter() {
        if pos.0 < min_r {
            min_r = pos.0;
        }
        if pos.0 > max_r {
            max_r = pos.0;
        }
        if pos.1 < min_c {
            min_c = pos.1;
        }
        if pos.1 > max_c {
            max_c = pos.1;
        }
    }

    let n_rows = (max_r - min_r + 2) as usize;
    let n_cols = (max_c - min_c + 2) as usize;
    println!("min_r: {}, max_r: {}, min_c: {}, max_c: {}", min_r, max_r, min_c, max_c);
    println!("n_rows: {}, n_cols: {}", n_rows, n_cols);

    let mut map: Vec<Vec<char>> = vec![vec!['.'; n_cols]; n_rows];
    for pos in marked_pos.iter() {
        map[(pos.0 - min_r + 1) as usize][(pos.1 - min_c + 1) as usize] = '#';
    }

    // find inside cell. 
    let mut inside_pos: Position = (2, 0);
    let mut hit_boundary = false;
    for (cidx, c) in map[2].iter().enumerate() {
        if *c == '#' {
            hit_boundary = true;
        }
        if *c == '.' && hit_boundary {
            inside_pos.1 = cidx as isize;
            break;
        }
    }

    // flood fill.
    let mut stack: Vec<Position> = Vec::new();
    stack.push(inside_pos);
    while let Some(pos) = stack.pop() {
        // mark
        map[pos.0 as usize][pos.1 as usize] = '#';
        if map[pos.0 as usize - 1][pos.1 as usize] != '#' { stack.push((pos.0 - 1, pos.1)) };
        if map[pos.0 as usize + 1][pos.1 as usize] != '#' { stack.push((pos.0 + 1, pos.1)) };
        if map[pos.0 as usize][pos.1 as usize - 1] != '#' { stack.push((pos.0, pos.1 - 1)); }
        if map[pos.0 as usize][pos.1 as usize + 1] != '#' { stack.push((pos.0, pos.1 + 1)); }
    }

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    
    // count marked cells
    map.iter().map(|row| row.iter().filter(|c| **c == '#').count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day18_sample.txt"), 62);
        assert_eq!(part1("files/day18_input.txt"), 50603);
        assert_eq!(part2("files/day18_sample.txt"), 952408144115);
        assert_eq!(part2("files/day18_input.txt"), 96556251590677);
    }
}