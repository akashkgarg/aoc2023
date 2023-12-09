use crate::common::get_lines;

fn next_num(nums: &Vec<isize>) -> isize {
    if nums.iter().all(|n| *n == 0) {
        0
    } else { 
        let diffs = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<isize>>();
        let next = next_num(&diffs) + nums.last().unwrap();
        next
    }
}

fn prev_num(nums: &Vec<isize>) -> isize {
    if nums.iter().all(|n| *n == 0) {
        0
    } else { 
        let diffs = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<isize>>();
        let prev = nums.first().unwrap() - prev_num(&diffs);
        prev
    }
}

fn part1(filename: &str) -> isize {
    let lines = get_lines(filename);

    let mut sum = 0;
    for line in lines.iter()  {
        let nums = line.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        let next = next_num(&nums);
        sum += next;
    }
    return sum;
}

fn part2(filename: &str) -> isize {
    let lines = get_lines(filename);

    let mut sum = 0;
    for line in lines.iter()  {
        let nums = line.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        let prev = prev_num(&nums);
        sum += prev;
    }
    return sum;
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day09_sample.txt"), 114);
        assert_eq!(part1("files/day09_input.txt"), 1980437560);
        assert_eq!(part2("files/day09_sample.txt"), 2);
        assert_eq!(part2("files/day09_input.txt"), 977);
    }
}