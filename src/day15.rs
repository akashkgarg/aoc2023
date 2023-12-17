use crate::common::get_lines;

// Each box contains a list of (name, focal_length) pairs. 
// We use an optional for when we need to remove elements in the middle of the Box
type Box = Vec<Option<(String, usize)>>;

fn compute_hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.bytes() {
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

fn part1(filename: &str) -> usize { 
    let lines = get_lines(filename);
    let mut sum = 0;
    for line in lines {
        for instr in line.split(',') {
            sum += compute_hash(instr);
        }
    }
    sum
}

fn part2(filename: &str) -> usize { 
    let lines = get_lines(filename);
    let mut total_power = 0;

    // always have 256 boxes
    let mut boxes: Vec<Box> = vec![Vec::new(); 256];

    for line in lines {
        for instr in line.split(',') {
            if let Some(idx) = instr.find('=') { 
                let (name, focal_length) = instr.split_at(idx);
                let focal_length = focal_length[1..].parse::<usize>().unwrap();
                let hash = compute_hash(name);
                // do I really need to clone x here to get the value? 
                if let Some(found_idx) = boxes[hash].iter().position(|x| x.is_some() && x.clone().unwrap().0 == name) { 
                    boxes[hash][found_idx].as_mut().unwrap().1 = focal_length;
                } else { 
                    boxes[hash].push(Some((name.to_string(), focal_length)));
                }
            } else if let Some(idx) = instr.find('-') {
                let (name, _) = instr.split_at(idx);
                let hash = compute_hash(name);
                if let Some(found_idx) = boxes[hash].iter().position(|x| x.is_some() && x.clone().unwrap().0 == name) { 
                    boxes[hash][found_idx] = None;
                }
            }
        }
    }

    // compute power for each box.
    for (box_idx, b) in boxes.iter().enumerate() { 
        let mut power = 0; 
        let mut slot = 1;
        for item in b { 
            if let Some((_, focal_length)) = item { 
                power += (box_idx + 1) * slot * focal_length;
                slot += 1;
            }
        }
        total_power += power;
    }

    total_power
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute_hash("HASH"), 52);
        assert_eq!(part1("files/day15_sample.txt"), 1320);
        assert_eq!(part1("files/day15_input.txt"), 506891);
        assert_eq!(part2("files/day15_sample.txt"), 145);
        assert_eq!(part2("files/day15_input.txt"), 230462);
    }
}