use crate::common::get_lines;

// count set bits using brian kernighan's algorithm
fn count_set_bits(n: usize) -> usize {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

fn is_valid_reflection(nums: &Vec<usize>, i: usize) -> bool {
    let mut left: isize = i as isize;
    let mut right: isize = i as isize + 1;
    while left >= 0 && right < nums.len() as isize {
        if nums[left as usize] != nums[right as usize] {
            return false;
        }
        left -= 1;
        right += 1;
    }
    true
}

fn find_reflection_nums(nums: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut j = i + 1;
    while j < nums.len() {
        if nums[i] == nums[j] && is_valid_reflection(nums, i) {
            return j
        }
        i += 1;
        j += 1;
    }
    0
}

// count # of smudges (bit flips) needed to make a valid reflection starting
// at index i.
fn count_smudges(nums: &Vec<usize>, i: usize) -> usize {
    let mut left: isize = i as isize;
    let mut right: isize = i as isize + 1;
    let mut count = 0;
    while left >= 0 && right < nums.len() as isize {
        let xor = nums[left as usize] ^ nums[right as usize];
        count += count_set_bits(xor);
        left -= 1;
        right += 1;
    }
    count
}

// return smudges needed to make a reflection work for all possible rows in 
// nums
fn find_reflection_smudges(nums: &Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut j = i + 1;
    let mut reflections: Vec<usize> = Vec::new();
    while j < nums.len() {
        let smudges = count_smudges(nums, i);
        reflections.push(smudges);
        i += 1;
        j += 1;
    }
    reflections
}

// convert each row / col into a usize
fn rows_and_cols(map: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut rows: Vec<usize> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();

    for row in map {
        let mut r = 0;
        for c in row {
            r <<= 1;
            if *c == '#' {
                r |= 1;
            }
        }
        rows.push(r);
    }

    let n_rows = rows.len();
    let n_cols = map[0].len();

    for i in 0..n_cols {
        let mut c = 0;
        for j in 0..n_rows {
            c <<= 1;
            if map[j][i] == '#' {
                c |= 1;
            }
        }
        cols.push(c);
    }
    (rows, cols)
}

fn find_reflections(map: &Vec<Vec<char>>) -> (usize, usize) {
    // encode each row and column as a usize, where each bit indicates 1 if #, 0
    // otherwise.
    let (rows, cols) = rows_and_cols(map);

    // find all reflections of rows and cols
    (find_reflection_nums(&cols), find_reflection_nums(&rows))
}

fn part1(filename: &str) -> usize {
    let lines = get_lines(filename);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            let (vertical, horizontal) = find_reflections(&map);
            println!("vertical: {}, horizontal: {}", vertical, horizontal);
            sum += vertical + horizontal*100;
            map.clear();
        } else { 
            let chars: Vec<char> = line.chars().collect();
            map.push(chars);
        }
    }
    sum
}

fn part2(filename: &str) -> usize {
    let lines = get_lines(filename);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            let (rows, cols)  = rows_and_cols(&map);
            let horizontal_smudges = find_reflection_smudges(&rows);
            let vertical_smudges = find_reflection_smudges(&cols);

            // find the index where only 1 smudge is needed
            let horizontal = horizontal_smudges.iter().position(|&x| x == 1);
            let vertical = vertical_smudges.iter().position(|&x| x == 1);
            if let Some(offset) = horizontal {
                sum += 100 * (offset + 1);
            }
            if let Some(offset) = vertical {
                sum += offset + 1;
            }
            map.clear();
        } else { 
            let chars: Vec<char> = line.chars().collect();
            map.push(chars);
        }
    }
    sum
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day13_sample.txt"), 405);
        assert_eq!(part1("files/day13_input.txt"), 37381);
        assert_eq!(part2("files/day13_sample.txt"), 400);
        assert_eq!(part2("files/day13_input.txt"), 28210);
    }
}