use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum MapElement {
    Symbol, 
    Gear,
    Digit(u32),
    Empty
}

fn read_map(filename: &str) -> Vec<Vec<MapElement>> {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");

    let lines = contents.lines();

    let mut map: Vec<Vec<MapElement>> = Vec::new();

    for line in lines {
        let mut row: Vec<MapElement> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(MapElement::Empty),
                '*' => row.push(MapElement::Gear),
                _ => match c.to_digit(10) {
                    Some(d) => row.push(MapElement::Digit(d)),
                    None => row.push(MapElement::Symbol),
                }
            }
        }
        map.push(row);
    }

    return map;
}

fn get_full_number(map: &Vec<Vec<MapElement>>, r: isize, c: isize) -> (u32, Vec<(isize, isize)>) {
    let mut number = 0;
    let mut j: isize = c;

    while j >= 0 && j < map[r as usize].len() as isize {
        match map[r as usize][j as usize] {
            MapElement::Digit(d) => { 
                j -= 1; 
            }
            _ => break,
        }
    }
    j += 1;

    let mut visited: Vec<(isize, isize)> = Vec::new();
    while j >= 0 && j < map[r as usize].len() as isize{
        match map[r as usize][j as usize] { 
            MapElement::Digit(d) => { 
                number = number * 10 + d; 
                visited.push((r, j));
                j += 1; 
            }
            _ => break,
        }
    }

    return (number, visited);
}

fn neighboring_numbers(map: &Vec<Vec<MapElement>>, r: isize, c: isize) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let mut visited_neighbors: HashSet<(isize, isize)> = HashSet::new();

    for i in r-1..r+2 {
        for j in c-1..c+2 {
            if i == r && j == c {
                continue;
            }
            if i < 0 || j < 0 || i >= map.len() as isize || j >= map[i as usize].len() as isize {
                continue;
            }
            if visited_neighbors.contains(&(i, j)) {
                continue;
            }
            match map[i as usize][j as usize] {
                MapElement::Digit(d) =>  {
                    let (num, visited) = get_full_number(map, i, j);
                    numbers.push(num);
                    for (r, c) in visited {
                        visited_neighbors.insert((r, c));
                    }
                },
                _ => continue,
            }
        }
    }
    return numbers;
}

fn part1(filename: &str) -> u32 {
    let map = read_map(filename);
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let element = map[r][c];
            sum += match element {
                MapElement::Symbol | MapElement::Gear => neighboring_numbers(&map, r as isize, c as isize).iter().sum(),
                _ => 0,
            };
        }
    }
    return sum;
}

fn part2(filename: &str) -> u32 {
    let map = read_map(filename);
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let element = map[r][c];
            sum += match element {
                MapElement::Gear =>  {
                    let numbers = neighboring_numbers(&map, r as isize, c as isize);
                    if numbers.len() == 2 {
                        numbers.iter().product()
                    } else {
                        0
                    }
                }
                _ => 0,
            };
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_number() {
        let map = read_map("files/day03_sample.txt");
        assert_eq!(get_full_number(&map, 0, 0).0, 467);
        assert_eq!(get_full_number(&map, 0, 1).0, 467);
        assert_eq!(get_full_number(&map, 0, 2).0, 467);
        assert_eq!(get_full_number(&map, 2, 2).0, 35);
        assert_eq!(get_full_number(&map, 2, 3).0, 35);
        assert_eq!(get_full_number(&map, 0, 3).0, 0);
    }

    #[test]
    fn test() {
        assert_eq!(part1("files/day03_sample.txt"), 4361);
        assert_eq!(part1("files/day03_input.txt"), 535078);
        assert_eq!(part2("files/day03_sample.txt"), 467835);
        assert_eq!(part2("files/day03_input.txt"), 75312571);
    }
}
