use crate::common::get_lines;

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum HandType {
    HighCard = 1, 
    OnePair = 2, 
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5, 
    FourOfAKind = 6,
    FiveOfAKind = 7
}

// this allows you do do hand.into()
impl From<HandType> for u8 {
    fn from(m: HandType) -> u8 {
        m as u8
    }
}

// store the 5 cards, it's score and the bid.
type HandScore = u32;
type Bid = usize;
type Hand = [u8; 5];

fn parse_hand(h: &str, j_is_joker: bool) -> Hand {
    let mut hand = [0; 5];
    for (i, c) in h.chars().enumerate() {
        match c {
            'A' => hand[i] = 14,
            'K' => hand[i] = 13,
            'Q' => hand[i] = 12,
            'J' => hand[i] = if j_is_joker { 1 } else { 11 },
            'T' => hand[i] = 10,
            _ => hand[i] = c.to_digit(10).unwrap() as u8
        }
    }
    return hand as Hand;
}

fn calc_hand_type(hand: Hand) -> HandType { 
    let mut counts = [0; 15];
    for c in hand.iter() {
        counts[*c as usize] += 1;
    }

    let mut pairs = 0;
    let mut triples = 0;
    let mut quads = 0;
    let mut fives = 0;
    for c in counts.iter() {
        if *c == 2 { pairs += 1; }
        if *c == 3 { triples += 1; }
        if *c == 4 { quads += 1; }
        if *c == 5 { fives += 1; }
    }

    let jokers = counts[1];
    
    if jokers == 1 { 
        if quads == 1 { return HandType::FiveOfAKind; }
        if triples == 1 { return HandType::FourOfAKind; }
        if pairs == 2 { return HandType::FullHouse; }
        if pairs == 1 { return HandType::ThreeOfAKind; }
        return HandType::OnePair;
    } else if jokers == 2 { 
        if triples == 1 { return HandType::FiveOfAKind; }
        if pairs == 2 { return HandType::FourOfAKind; } // one of these pairs is a joker pair
        return HandType::ThreeOfAKind;
    } else if jokers == 3 { 
        if pairs == 1 { return HandType::FiveOfAKind; }
        return HandType::FourOfAKind;
    } else if jokers == 4 { 
        return HandType::FiveOfAKind;
    } else { 
        if fives == 1 { return HandType::FiveOfAKind; }
        if quads == 1 { return HandType::FourOfAKind; }
        if triples == 1 && pairs == 1 { return HandType::FullHouse; }
        if triples == 1 { return HandType::ThreeOfAKind; }
        if pairs == 2 { return HandType::TwoPair; }
        if pairs == 1 { return HandType::OnePair; }
        return HandType::HighCard;
    }
}

// 5 cards * 4 bits = 20 bits to keep track of card ordering. 
// 8 bits for the HandType. 
// This score can be used to sort the hands in the game.
fn calc_hand_score(hand: Hand) -> HandScore {
    let mut score = calc_hand_type(hand) as HandScore;
    for c in hand.iter() {
        score = (score << 4) | *c as HandScore;
    }
    // println!("{:#01x}", score);
    score
}

fn solve(filename: &str, with_joker: bool) -> usize { 
    let lines = get_lines(filename);

    // store the hand, the score and the bid.
    let mut hands: Vec<(Hand, HandScore, Bid)> = Vec::new();
    for line in lines.iter() {
        let mut line_iter = line.split_whitespace();
        let hand = parse_hand(line_iter.next().unwrap(), with_joker);
        let bid = line_iter.next().unwrap().parse::<usize>().unwrap();
        let score = calc_hand_score(hand);
        hands.push((hand, score, bid));
    }

    hands.sort_by(|a, b| a.1.cmp(&b.1));
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i + 1) * hand.2;
    }
    return sum;
}

fn part1(filename: &str) -> usize {
    solve(filename, false)
}

fn part2(filename: &str) -> usize {
    solve(filename, true)
}

mod tests {
    use super::*;

    #[test]
    fn test_hand_score() {
        assert_eq!(calc_hand_score(parse_hand("AAAAA", false)), 0x7EEEEE);
        assert_eq!(calc_hand_score(parse_hand("KTJJT", false)), 0x3DABBA);
        assert_eq!(calc_hand_score(parse_hand("23456", false)), 0x123456);
        assert_eq!(calc_hand_score(parse_hand("JKKK2", true)) < calc_hand_score(parse_hand("QQQQ2", true)), true);
    }

    #[test]
    fn test_hand_type_with_joker() {
        assert_eq!(calc_hand_type(parse_hand("AAAAA", true)), HandType::FiveOfAKind);
        assert_eq!(calc_hand_type(parse_hand("AAAJJ", true)), HandType::FiveOfAKind);
        assert_eq!(calc_hand_type(parse_hand("JJJJJ", true)), HandType::FiveOfAKind);
        assert_eq!(calc_hand_type(parse_hand("JJJJT", true)), HandType::FiveOfAKind);
        assert_eq!(calc_hand_type(parse_hand("QJJQ2", true)), HandType::FourOfAKind);
        assert_eq!(calc_hand_type(parse_hand("23456", true)), HandType::HighCard);
        assert_eq!(calc_hand_type(parse_hand("2345J", true)), HandType::OnePair);
        assert_eq!(calc_hand_type(parse_hand("JKKK2", true)), HandType::FourOfAKind);
        assert_eq!(calc_hand_type(parse_hand("QQQQ2", true)), HandType::FourOfAKind);
    }

    #[test]
    fn test() {
        assert_eq!(part1("files/day07_sample.txt"), 6440);
        assert_eq!(part1("files/day07_input.txt"), 252295678);
        assert_eq!(part2("files/day07_sample.txt"), 5905);
        assert_eq!(part2("files/day07_input.txt"), 250577259);
    }
}