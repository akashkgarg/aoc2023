fn part1(filename: &str) -> i32 {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");


    let lines = contents.lines();

    let mut sum: i32 = 0;

    for line in lines {
        let mut ds: Vec<i32> = Vec::new();
        for c in line.chars() {
            match c.to_digit(10) {
                Some(d) => ds.push(d as i32),
                _ => continue,
            }
        }
        sum += ds.first().unwrap() * 10 + ds.last().unwrap();
    }

    return sum;
}

fn part2(filename: &str)  -> i32 {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");

    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let lines = contents.lines();

    let mut sum: i32 = 0;

    for line in lines {
        // store the index and digit as a pair
        let mut ds: Vec<(usize, i32)> = Vec::new();
        for (idx, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                Some(d) => ds.push((idx, d as i32)),
                // try reading the digit as a word
                _ => {
                    for (d, word) in numbers.iter().enumerate() {
                        if line[idx..].starts_with(word) {
                            ds.push((idx, d as i32));
                            break;
                        }
                    }
                },
            }
        }
        // sort our digits by index
        ds.sort_by(|a, b| a.0.cmp(&b.0));

        sum += ds.first().unwrap().1 * 10 + ds.last().unwrap().1;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        assert_eq!(part1("files/day01_sample.txt"), 142);
        assert_eq!(part1("files/day01_input.txt"), 54667);
        assert_eq!(part2("files/day01_sample2.txt"), 281);
        assert_eq!(part2("files/day01_input.txt"), 54203);
    }
}
