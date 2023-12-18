use crate::common::get_lines;
use std::collections::HashSet; 

type Direction = (isize, isize);
type Position = (isize, isize);

fn parse_map(filename: &str) -> Vec<Vec<char>> {
    let lines = get_lines(filename);
    let mut map = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }
    map
}

fn move_beams(map: &Vec<Vec<char>>, beams: &mut HashSet<(Position, Direction)>, visited: &mut Vec<Vec<HashSet<Direction>>>) {
    let mut new_beams: HashSet<(Position, Direction)> = HashSet::new();

    for beam in beams.iter() {
        let (pos, dir) = beam;
        let (r, c) = *pos;
        let (dr, dc) = *dir;
        let (nr, nc) = (r + dr, c + dc);
        if nr < 0 || nr as usize >= map.len() || nc < 0 || nc as usize >= map[0].len() {
            // kill beam out of bounds.
            continue;
        }
        let x = map[nr as usize][nc as usize];
        if x == '/' {
            new_beams.insert(((nr, nc), (-dc, -dr)));
        } else if x == '\\' {
            new_beams.insert(((nr, nc), (dc, dr)));
        } else if x == '|' { 
            if dc == 0 { 
                // moving up and down, no split, just keep going
                new_beams.insert(((nr, nc), (dr, dc)));
            } else { 
                // moving left or right, split into two beams
                new_beams.insert(((nr, nc), (dc, dr)));
                new_beams.insert(((nr, nc), (-dc, -dr)));
            }
        } else if x == '-' {
            if dr == 0 { 
                // moving left and right, no split, just keep going
                new_beams.insert(((nr, nc), (dr, dc)));
            } else { 
                // moving up or down, split into two beams
                new_beams.insert(((nr, nc), (dc, dr)));
                new_beams.insert(((nr, nc), (-dc, -dr)));
            }
        } else {
            // continue along. 
            new_beams.insert(((nr, nc), (dr, dc)));
        }
    }

    // mark visited and replace beams with new beams.
    beams.clear();
    for beam in new_beams.iter() {
        let (pos, dir) = beam;
        let (r, c) = *pos;
        let (dr, dc) = *dir;
        if visited[r as usize][c as usize].contains(&(dr, dc)) {
            // kill beam if it's already been here.
            continue;
        }
        visited[r as usize][c as usize].insert((dr, dc));
        beams.insert(*beam);
    }
}

fn print_visited(visited: &Vec<Vec<HashSet<Direction>>>) {
    for row in visited {
        for col in row {
            if !col.is_empty() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_energized_tiles(map: &Vec<Vec<char>>, pos: Position, dir: Direction) -> usize {
    // keep visited locations in the map, each location in map denotes which 
    // directions light has traveled through it. 
    let mut visited: Vec<Vec<HashSet<Direction>>> = vec![vec![HashSet::new(); map[0].len()]; map.len()];    

    // keep track of all existing beam positions and directon it's traveling in, 
    // there may be multiple light rays.
    let mut beams: HashSet<(Position, Direction)> = HashSet::new();

    // starting position and direction
    beams.insert((pos, dir));

    while !beams.is_empty() {
        move_beams(&map, &mut beams, &mut visited);
    }
    // print_visited(&visited);

    visited.iter().map(|row| row.iter().filter(|col| !col.is_empty()).count()).sum()
}

fn part1(filename: &str) -> usize { 
    let map = parse_map(filename);
    find_energized_tiles(&map, (0, -1), (0, 1))
}

fn part2(filename: &str) -> usize { 
    let map = parse_map(filename);
    let mut energies = Vec::new();
    // left edges
    for i in 0..map.len() {
        energies.push(find_energized_tiles(&map, (i as isize, -1), (0, 1)));
    }

    // right edges
    for i in (0..map.len()).rev() { 
        energies.push(find_energized_tiles(&map, (i as isize, map[0].len() as isize), (0, -1)));
    }

    // top edges
    for i in 0..map[0].len() {
        energies.push(find_energized_tiles(&map, (-1, i as isize), (1, 0)));
    }

    // bottom edges
    for i in (0..map[0].len()).rev() { 
        energies.push(find_energized_tiles(&map, (map.len() as isize, i as isize), (-1, 0)));
    }

    *energies.iter().max().unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day16_sample.txt"), 46);
        assert_eq!(part1("files/day16_input.txt"), 7067);
        assert_eq!(part2("files/day16_sample.txt"), 51);
        assert_eq!(part2("files/day16_input.txt"), 7324);
    }
}