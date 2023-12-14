use crate::common::get_lines;

fn parse_map(filename: &str, expansion: usize) -> (Vec<Vec<(char, usize, usize)>>, Vec<(usize, usize)>) {
    let lines = get_lines(filename);
    // keep a list of (char, x, y) tuples where x is cost of 
    // traversing current row, y is cost of traversing current column.
    let mut map: Vec<Vec<(char, usize, usize)>> = Vec::new();
    let mut galaxy_positions = Vec::new();
    for line in lines {
        let row_cost: usize = if line.chars().all(|c| c == '.') { expansion } else { 1 };
        let mut row = Vec::new();
        for c in line.chars() {
            if c == '#' {
                galaxy_positions.push((map.len(), row.len()));
            }
            row.push((c, row_cost, 1));
        }
        map.push(row);
    }

    // find empty columns and update their costs.
    for c in 0..map[0].len() { 
        let mut empty = true;
        let col_cost = if map.iter().enumerate().all(|(r, _)| map[r][c].0 == '.') { expansion } else { 1 };
        for r in 0..map.len() {
            map[r][c].2 = col_cost;
        }
    }
    return (map, galaxy_positions);
}

fn solve(filename: &str, expansion: usize) -> usize {
    let (map, galaxy_positions) = parse_map(filename, expansion);
    //println!("{:?}", map);
    let mut sum = 0;
    for i in 0..galaxy_positions.len() {
        for j in i+1..galaxy_positions.len() {
            let (r1, c1) = galaxy_positions[i];
            let (r2, c2) = galaxy_positions[j];
            let from_r = if r1 < r2 { r1 } else { r2 };
            let to_r = if r1 < r2 { r2 } else { r1 };
            let from_c = if c1 < c2 { c1 } else { c2 };
            let to_c = if c1 < c2 { c2 } else { c1 };

            // compute cost of manhattan path.
            let mut cost = 0;
            for r in from_r..to_r {
                cost += map[r][from_c].1;
            }
            for c in from_c..to_c {
                cost += map[from_r][c].2;
            }
            //println!("from {} to {} cost {}", i+1, j+1, cost);
            sum += cost;
        }
    }
    return sum;
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve("files/day11_sample.txt", 2), 374);
        assert_eq!(solve("files/day11_input.txt", 2), 9769724);
        assert_eq!(solve("files/day11_sample.txt", 10), 1030);
        assert_eq!(solve("files/day11_sample.txt", 100), 8410);
        assert_eq!(solve("files/day11_input.txt", 1000000), 374);
    }
}