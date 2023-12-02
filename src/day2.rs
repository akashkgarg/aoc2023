use std::cmp::max;

fn part1(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");

    let lines = contents.lines();

    let mut sum: usize = 0;

    for line in lines {
        let mut game_split = line.split(": ");

        let game = game_split.next().unwrap();
        let game_index = game.split_whitespace().collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();

        let draws = game_split.next().unwrap();
        let mut is_valid = true;

        draws.split("; ").for_each(|draw| {
            let draw_split = draw.split(", ").collect::<Vec<&str>>();
            for color_count in draw_split.into_iter() {
                let count_color_pair = color_count.split_whitespace().collect::<Vec<&str>>();
                let count = count_color_pair[0].parse::<usize>().unwrap();
                let color = count_color_pair[1];
                match color {
                    "red" => {
                        if count > 12 {
                            is_valid = false;
                        }
                    },
                    "green" => {
                        if count > 13 {
                            is_valid = false;
                        }
                    },
                    "blue" => {
                        if count > 14 {
                            is_valid = false;
                        }
                    },
                    _ => {
                        println!("found non-matching color, {}", color)
                    },
                }
            }
        });
        if is_valid {
            sum += game_index;
        }
    }

    return sum;
}

fn part2(filename: &str) -> usize {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");

    let lines = contents.lines();

    let mut sum: usize = 0;

    for line in lines {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        let mut game_split = line.split(": ");

        let _game = game_split.next().unwrap();

        let draws = game_split.next().unwrap();

        draws.split("; ").for_each(|draw| {
            let draw_split = draw.split(", ").collect::<Vec<&str>>();
            for color_count in draw_split.into_iter() {
                let count_color_pair = color_count.split_whitespace().collect::<Vec<&str>>();
                let count = count_color_pair[0].parse::<usize>().unwrap();
                let color = count_color_pair[1];
                match color {
                    "red" => {
                        red_max = max(red_max, count);
                    },
                    "green" => {
                        green_max = max(green_max, count);
                    },
                    "blue" => {
                        blue_max = max(blue_max, count);
                    },
                    _ => {
                        println!("found non-matching color, {}", color)
                    },
                }
            }
        });
        sum += red_max * green_max * blue_max;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        assert_eq!(part1("files/day02_sample.txt"), 8);
        assert_eq!(part1("files/day02_input.txt"), 1931);
        assert_eq!(part2("files/day02_sample.txt"), 2286);
        assert_eq!(part2("files/day02_input.txt"), 83105);
    }
}
