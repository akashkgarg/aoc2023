use crate::common::get_lines;

fn parse_map(filename: &str) -> ((usize, usize), Vec<Vec<char>>) { 
    let lines = get_lines(filename);
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos = (0, 0);
    for line in lines.iter() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
            if ch == 'S' {
                start_pos = (map.len(), row.len() - 1);
            }
        }
        map.push(row);
    }
    return (start_pos, map);
}

fn get_starting_neighbors(map: &Vec<Vec<char>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let ch = map[r][c];

    let valid_up = |c: char| -> bool { c == 'S' || c == '|' || c == '7' || c == 'F' };
    let valid_down = |c: char| -> bool { c == 'S' || c == '|' || c == 'J' || c == 'L' };
    let valid_left = |c: char| -> bool { c == 'S' || c == '-' || c == 'L' || c == 'F' };
    let valid_right = |c: char| -> bool { c == 'S' || c == '-' || c == 'J' || c == '7' };

    match ch {
        'S' => { 
            if r > 0 && valid_up(map[r - 1][c]) {
                neighbors.push((r - 1, c));
            }
            if r < map.len() - 1 && valid_down(map[r + 1][c]) {
                neighbors.push((r + 1, c));
            }
            if c > 0 && valid_left(map[r][c - 1]) {
                neighbors.push((r, c - 1));
            }
            if c < map[0].len() - 1 && valid_right(map[r][c + 1]) {
                neighbors.push((r, c + 1));
            }
        },
        _ => { panic!("not given a start token!") }
    }
    return neighbors;
}

type Position = (usize, usize); 

fn get_next(prev: Position, curr: Position, map: &Vec<Vec<char>>) -> Option<Position> {
    let (r, c) = curr;
    let ch = map[r][c];

    let valid_up = |c: char| -> bool { c == 'S' || c == '|' || c == '7' || c == 'F' };
    let valid_down = |c: char| -> bool { c == 'S' || c == '|' || c == 'J' || c == 'L' };
    let valid_left = |c: char| -> bool { c == 'S' || c == '-' || c == 'L' || c == 'F' };
    let valid_right = |c: char| -> bool { c == 'S' || c == '-' || c == 'J' || c == '7' };

    match ch {
        '|' => { 
            if r > 0 && valid_up(map[r - 1][c]) && (r - 1, c) != prev {
                return Some((r - 1, c));
            }
            if r < map.len() - 1 && valid_down(map[r + 1][c]) && (r + 1, c) != prev {
                return Some((r + 1, c));
            }
        },
        '-' => { 
            if c > 0 && valid_left(map[r][c - 1]) && (r, c - 1) != prev {
                return Some((r, c - 1));
            }
            if c < map[0].len() - 1 && valid_right(map[r][c + 1]) && (r, c + 1) != prev  {
                return Some((r, c + 1));
            }
        },
        '7' => { 
            if c > 0 && valid_left(map[r][c - 1]) && (r, c - 1) != prev { 
                return Some((r, c - 1));
            }
            if r < map.len() - 1 && valid_down(map[r + 1][c]) && (r + 1, c) != prev {
                return Some((r + 1, c));
            }
        },
        'J' => { 
            if c > 0 && valid_left(map[r][c - 1]) && (r, c - 1) != prev { 
                return Some((r, c - 1));
            }
            if r > 0 && valid_up(map[r - 1][c]) && (r - 1, c) != prev {
                return Some((r - 1, c));
            }
        },
        'L' => { 
            if c < map[0].len() - 1 && valid_right(map[r][c + 1]) && (r, c + 1) != prev {
                return Some((r, c + 1));
            }
            if r > 0 && valid_up(map[r - 1][c]) && (r - 1, c) != prev {
                return Some((r - 1, c));
            }
        }, 
        'F' => { 
            if c < map[0].len() - 1 && valid_right(map[r][c + 1]) && (r, c + 1) != prev {
                return Some((r, c + 1));
            }
            if r < map.len() - 1 && valid_down(map[r + 1][c]) && (r + 1, c) != prev {
                return Some((r + 1, c));
            }
        },
        _ => {  }
    }
    return None;
}

fn find_path(start_pos: Position, map: &Vec<Vec<char>>) -> Vec<Position> {
    let mut path: Vec<Position> = Vec::new();

    for path_start in get_starting_neighbors(&map, start_pos.0, start_pos.1) {
        let mut prev = start_pos;
        path.push(path_start);
        while let Some(next) = get_next(prev, path[path.len() - 1], &map) {
            path.push(next);
            prev = path[path.len() - 2];
            if map[next.0][next.1] == 'S' {
                return path;
            }
        }
        // no path found that loops to 'S'
        path.clear();
    }
    return path;
}

fn part1(filename: &str) -> usize {
    let (start_pos, map) = parse_map(filename);
    let path = find_path(start_pos, &map);
    return path.len() / 2;
}

#[allow(dead_code)]
fn print_path_in_map(path: &Vec<Position>, map: &Vec<Vec<char>>) {
    let mut map_copy = map.clone();
    for row in map_copy.iter_mut() {
        for col in row.iter_mut() {
            if *col != 'I' && *col != 'O' {
                *col = '.';
            }
        }
    }
    for pos in path.iter() {
        map_copy[pos.0][pos.1] = map[pos.0][pos.1];
    }
    for row in map_copy.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part2(filename: &str) -> usize {
    let (start_pos, map) = parse_map(filename);
    let mut map_copy = map.clone();
    let path = find_path(start_pos, &map);
    let path_set = path.iter().cloned().collect::<std::collections::HashSet<_>>();

    // clear the map_copy and mark the path
    for row in map_copy.iter_mut() {
        for col in row.iter_mut() {
            *col = '.';
        }
    }
    for pos in path.iter() {
        map_copy[pos.0][pos.1] = map[pos.0][pos.1];
    }


    let mut inside_positions = Vec::new();
    let mut outside_positions = Vec::new();

    let mut mark_map = |pos: &Position, outside_offset: (isize, isize), inside_offset: (isize, isize)| {
        let out_pos = (pos.0 as isize + outside_offset.0, pos.1 as isize + outside_offset.1);
        let in_pos = (pos.0 as isize + inside_offset.0, pos.1 as isize + inside_offset.1);
        if out_pos.0 >= 0 && out_pos.0 < map_copy.len() as isize && 
           out_pos.1 >= 0 && out_pos.1 < map_copy[0].len() as isize && 
           !path_set.contains(&(out_pos.0 as usize, out_pos.1 as usize)) {
            outside_positions.push((out_pos.0 as usize, out_pos.1 as usize));
            map_copy[out_pos.0 as usize][out_pos.1 as usize] = 'O';
        } 
        if in_pos.0 >= 0 && in_pos.0 < map_copy.len() as isize && 
           in_pos.1 >= 0 && in_pos.1 < map_copy[0].len() as isize && 
           !path_set.contains(&(in_pos.0 as usize, in_pos.1 as usize)) {
            inside_positions.push((in_pos.0 as usize, in_pos.1 as usize));
            map_copy[in_pos.0 as usize][in_pos.1 as usize] = 'I';
        }
    };

    // find the starting inside/outside offset
    let diff = (path[0].0 as isize - start_pos.0 as isize, path[0].1 as isize - start_pos.1 as isize);
    let mut inside_offset = (diff.1, diff.0);
    let mut outside_offset = (-diff.1, -diff.0);

    // track the inside/outside offsets and mark along the path
    let mut path_iter = path.iter();
    while let Some(pos) = path_iter.next() { 
        let ch = map[pos.0][pos.1];
        if ch == 'S' {
            break;
        }
        //println!("pos: {:?}, outside: {:?}, inside: {:?}", pos, outside_offset, inside_offset);
        mark_map(pos, outside_offset, inside_offset);
        if ch == 'F' || ch == 'J' {
            // swap to maintain winding order
            outside_offset = (outside_offset.1, outside_offset.0);
            inside_offset = (inside_offset.1, inside_offset.0);
            // mark again once we've rotated on the path
            mark_map(pos, outside_offset, inside_offset);
        } else if ch == 'L' || ch == '7' {
            // swap and flip to maintain winding order
            outside_offset = (-outside_offset.1, -outside_offset.0);
            inside_offset = (-inside_offset.1, -inside_offset.0);
            // mark again once we've rotated on the path
            mark_map(pos, outside_offset, inside_offset);
        }
    }

    let mut flood_fill = |pos: Position, ch: char| {
        let mut stack = vec![pos];
        while let Some(pos) = stack.pop() {
            map_copy[pos.0][pos.1] = ch;
            if pos.0 > 0 && map_copy[pos.0 - 1][pos.1] == '.' { stack.push((pos.0 - 1, pos.1)); }
            if pos.0 < map_copy.len() - 1 && map_copy[pos.0+1][pos.1] == '.' { stack.push((pos.0 + 1, pos.1)); }
            if pos.1 > 0 && map_copy[pos.0][pos.1-1] == '.' { stack.push((pos.0, pos.1 - 1)); }
            if pos.1 < map_copy[0].len() - 1 && map_copy[pos.0][pos.1 + 1] == '.' { stack.push((pos.0, pos.1 + 1)); }
        }
    };

    // flood fill
    for pos in inside_positions.iter() {
        flood_fill(*pos, 'I');
    }
    for pos in outside_positions.iter() {
        flood_fill(*pos, 'O');
    }

    // count the O's and I's n map_copy
    let mut outside_count = 0;
    let mut inside_count = 0;
    let mut dot_count = 0;
    for row in map_copy.iter() {
        for col in row.iter() {
            if *col == 'O' {
                outside_count += 1;
            } else if *col == 'I' {
                inside_count += 1;
            } else if *col == '.' {
                dot_count += 1;
            }
        }
    }

    // print_path_in_map(&path, &map_copy);
    println!("outside: {}, inside: {}, unvisited: {}", outside_count, inside_count, dot_count);
    if dot_count > 0 {
        panic!("Found unvisited positions!");
    }
    std::cmp::min(outside_count, inside_count)
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day10_sample.txt"), 4);
        assert_eq!(part1("files/day10_sample2.txt"), 8);
        assert_eq!(part1("files/day10_input.txt"), 6951);
        assert_eq!(part2("files/day10_sample3.txt"), 4);
        assert_eq!(part2("files/day10_sample4.txt"), 8);
        assert_eq!(part2("files/day10_sample5.txt"), 10);
        assert_eq!(part2("files/day10_input.txt"), 563);
    }
}