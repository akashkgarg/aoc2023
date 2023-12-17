use crate::common::get_lines;
use std::collections::VecDeque;
use std::collections::HashMap;

// find next valid arrangement for n springs starting at offset
// if found, return the index of the next startin offset.
fn find_arrangement(chars: &Vec<char>, offset: usize, n: usize) -> Option<usize> {
    for i in offset..=chars.len() - n {
        if chars[i..i+n].iter().all(|c| *c == '#' || *c == '?') { 
            if i+n == chars.len() || chars[i+n] == '.' || chars[i+n] == '?' {
                return Some(i + n + 1);
            }
        }
    }
    None
}

// find all arragements for all nums starting at num_offset within chars starting at offset.
fn find_all_arrangements(chars: &Vec<char>, offset: usize, nums: &VecDeque<usize>, num_offset: usize, 
                         cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let mut count = 0;

    // memoize! 
    if cache.contains_key(&(offset, num_offset)) {
        return *cache.get(&(offset, num_offset)).unwrap();
    }

    if num_offset < nums.len() {
        let n = nums[num_offset];
        let mut i = offset;
        // try all possible arrangements for this num
        while i < chars.len() {
            if chars[i] == '.' {
                i += 1;
                continue;
            }

            if let Some(j) = find_arrangement(chars, i, n) {

                // don't continue if we have a spring before current one.
                if chars[offset..j-n-1].iter().any(|c| *c == '#') {
                    break;
                }

                // recurse
                count += find_all_arrangements(chars, j, nums, num_offset + 1, cache);
                // update i to beginning of this arragement + 1
                i = j - n;
            } else { 
                // go to next index
                i += 1;
            }
        }
    } else { 
        // ensure that we have no trailing springs either. 
        if offset < chars.len() && chars[offset..].iter().any(|c| *c == '#') {
            return 0;
        }
        return 1 // processed all nums, count as a valid arrangement
    }

    cache.insert((offset, num_offset), count);
    return count
}

fn find_arrangements_in_line(line: &str, repeat: usize) -> usize {
    let mut tokens = line.split_whitespace();
    let group = tokens.next().unwrap();
    let mut chars: Vec<char> = group.chars().collect();

    let num_str = tokens.next().unwrap().split(',');
    let mut nums = VecDeque::<usize>::new();
    for num in num_str {
        nums.push_back(num.parse::<usize>().unwrap());
    }

    // extend the group and nums repeat times
    let nums_copy = nums.clone();
    for _ in 0..repeat {
        chars.push('?');
        for c in group.chars() {
            chars.push(c);
        }
        nums.extend(nums_copy.clone());
    }

    // println!("chars: {:?}, nums: {:?}", chars, nums);
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    find_all_arrangements(&chars, 0, &nums, 0, &mut cache)
}

fn solve(filename: &str, repeat: usize) -> usize { 
    let lines = get_lines(filename);

    let mut count = 0;
    for line in lines {
        let c = find_arrangements_in_line(&line, repeat);
        count += c
    }

    count
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_arrangements_in_line("?...??.#.????##??? 1,1,1,7", 0), 9);
        assert_eq!(find_arrangements_in_line("??.???#?????#???. 1,13", 0), 2);
        assert_eq!(find_arrangements_in_line("?###???????? 3,2,1", 0), 10);
        assert_eq!(find_arrangements_in_line("????#??.???#????? 2,8", 0), 4);
        assert_eq!(find_arrangements_in_line("???.### 1,1,3", 4), 1);
        assert_eq!(find_arrangements_in_line("?###???????? 3,2,1", 4), 506250);
         
        assert_eq!(solve("files/day12_sample.txt", 0), 21);
        assert_eq!(solve("files/day12_input.txt", 0), 7090);
        assert_eq!(solve("files/day12_sample.txt", 4), 525152);
        assert_eq!(solve("files/day12_input.txt", 4), 6792010726878);
    }
}