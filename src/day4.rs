use crate::common::get_lines;
use std::collections::HashSet;

fn parse_card(card: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut parts = card.split(": ");
    let mut winning_nums: HashSet<u32> = HashSet::new();
    let mut our_nums: HashSet<u32> = HashSet::new();

    parts.next();
    let mut number_sections = parts.next().unwrap().split(" | ");

    let winning_section = number_sections.next().unwrap();
    for num in winning_section.split_whitespace() {
        winning_nums.insert(num.parse::<u32>().unwrap());
    }
    let our_section = number_sections.next().unwrap();
    for num in our_section.split_whitespace() {
        our_nums.insert(num.parse::<u32>().unwrap());
    }

    return (winning_nums, our_nums);
}

fn card_score(winning_nums: &HashSet<u32>, our_nums: &HashSet<u32>) -> u32 {
    let intersection = winning_nums.intersection(our_nums);
    let num_intersection = intersection.count();
    if num_intersection == 0 {
        return 0;
    }
    let exponent = num_intersection - 1;
    return 1 << exponent;
}

fn part1(filename: &str) -> u32 {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");

    let lines = contents.lines();

    let mut sum = 0;
    for card in lines {
        let (winning_nums, our_nums) = parse_card(card);
        sum += card_score(&winning_nums, &our_nums);
    }
    return sum
}

fn part2(filename: &str) -> u32 {
    let lines = get_lines(filename);

    let card_count = lines.len();

    // keep track of # of instances for each card
    let mut card_instances = vec![0; card_count];

    for (i, card_line) in lines.iter().enumerate() {
        card_instances[i] += 1;

        let (winning_nums, our_nums) = parse_card(card_line);
        let n = winning_nums.intersection(&our_nums).count();

        // Each instance of this card creates as many instances
        // for each of the next n cards.
        for card_idx in 0..n {
            card_instances[i + card_idx + 1] += card_instances[i];
        }
    }

    return card_instances.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let (winning_nums, our_nums) = parse_card("Card 1: 2 3 | 3 2");
        assert_eq!(winning_nums, [2, 3].into());
        assert_eq!(our_nums, [3, 2].into());
    }

    #[test]
    fn test_card_score() {
        {
            let (winning_nums, our_nums) = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
            assert_eq!(card_score(&winning_nums, &our_nums), 8);
        }

        {
            let (winning_nums, our_nums) = parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
            assert_eq!(card_score(&winning_nums, &our_nums), 0);
        }
    }

    #[test]
    fn test() {
        assert_eq!(part1("files/day04_sample.txt"), 13);
        assert_eq!(part1("files/day04_input.txt"), 28750);
        assert_eq!(part2("files/day04_sample.txt"), 30);
        assert_eq!(part2("files/day04_input.txt"), 10212704);
    }
}
