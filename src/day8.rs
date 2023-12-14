use crate::common::get_lines;
use std::collections::HashMap;
use regex::Regex;
use num::integer;

fn node_to_id(s: &str) -> usize {
    let mut num: usize = 0;
    for ch in s.chars() {
        num = num << 8 | ch as usize;
    }
    return num;
}

fn id_to_str(id: usize) -> String {
    let mut num = id;
    let mut s = String::new();
    while num > 0 {
        s.push((num & 0xFF) as u8 as char);
        num = num >> 8;
    }
    return s.chars().rev().collect::<String>();
}

fn parse_input(filename: &str) -> (Vec<usize>, HashMap<usize, [usize; 2]>) {
    let lines = get_lines(filename);
    let mut line_iter = lines.iter();

    let direction = line_iter.next().unwrap(); 

    let mut dirs: Vec<usize> = Vec::new(); 
    for ch in direction.chars() {
        match ch {
            'L' => dirs.push(0),
            'R' => dirs.push(1),
            _ => panic!("invalid direction"),
        }
    }

    line_iter.next(); // skip empty line

    // println!("driection = {}", direction);

    let re = Regex::new(r"([A-Z]+).*([A-Z][A-Z][A-Z]).*([A-Z][A-Z][A-Z]).*$").unwrap();

    let mut map: HashMap<usize, [usize; 2]> = HashMap::new();
    for line in line_iter { 
        // println!("line = {}", line);
        let Some(caps) = re.captures(line) else { println!("{}, no match", line); continue; };
        // println!("caps = {:?}", caps);
        let from = node_to_id(&caps[1]);
        let to = [ node_to_id(&caps[2]), node_to_id(&caps[3]) ];
        map.insert(from, to);
    }
    // println!("map = {:?}", map);
    return (dirs, map);
}

#[allow(dead_code)]
fn part2_bruteforce(filename: &str) -> usize { 
    let (dirs, map) = parse_input(filename);

    // find all nodes that end in A.
    let mut curr_nodes = map.iter().filter(|(k,_)| -> bool {
        (*k & 0xFF) == 'A' as usize
    }).map(|(k,_)| -> usize { *k }).collect::<Vec<usize>>();

    let mut curr_idx = 0;
    let n = dirs.len();

    let mut n_steps = 0;
    while !curr_nodes.iter().all(|node| -> bool { (node & 0xFF) == 'Z' as usize }) { 
        for i in 0..curr_nodes.len() {
            let curr = map.get(&curr_nodes[i]).unwrap()[dirs[curr_idx]];
            curr_nodes[i] = curr;
        }
        n_steps += 1;
        curr_idx = (curr_idx + 1) % n;
    }

    return n_steps;
}

// advance node to the next end node. Return the new ending node + steps it took to get there. 
fn steps_to_end((node, node_loc): (usize, usize), 
                dirs: &Vec<usize>, map: &HashMap<usize, [usize; 2]>) -> (usize, usize) {
    let mut curr = node;
    let n = dirs.len();

    let mut dir_loc = node_loc % n;

    let mut n_steps = 0;

    // advance
    curr = map.get(&curr).unwrap()[dirs[dir_loc]];
    n_steps += 1;
    dir_loc = (dir_loc + 1) % n;

    // iterate until we have a node that ends in Z.
    while (curr & 0xFF) != 'Z' as usize { 
        curr = map.get(&curr).unwrap()[dirs[dir_loc]];
        n_steps += 1;
        dir_loc = (dir_loc + 1) % n;
    }

    return (curr, n_steps);
}

fn part2_withlcm(filename: &str) -> usize {
    let (dirs, map) = parse_input(filename);

    // the starting nodes.
    let start_nodes = map.iter().filter(|(k,_)| -> bool {
        (*k & 0xFF) == 'A' as usize
    }).map(|(k,_)| -> usize { *k }).collect::<Vec<usize>>();

    // store nodes as current location and steps taken to get there. 
    let mut curr_nodes: Vec<(usize, usize)> = Vec::new();
    for node in start_nodes.iter() {
        curr_nodes.push((*node, 0));
    }

    // advance each node to the end. 
    for i in 0..curr_nodes.len() {
        let (node, steps) = curr_nodes[i];
        curr_nodes[i] = steps_to_end((node, steps), &dirs, &map);
    }

    // verify that each node cycles back to itself.
    for i in 0..curr_nodes.len() {
        let (node, location) = curr_nodes[i];
        let (new_node, advanced_by) = steps_to_end((node, location), &dirs, &map);
        if new_node != node {
            panic!("node does not repeat itself!")
        }
        if advanced_by != location {
            panic!("node is not cyclic without an offset!")
        }
    }

    // can we do a reduce or fold here? 
    let mut lcm = curr_nodes[0].1;
    for i in 1..curr_nodes.len() {
        lcm = integer::lcm(lcm, curr_nodes[i].1);
    }
    return lcm;
}

fn part2(filename: &str) -> usize { 
    let (dirs, map) = parse_input(filename);
    let n_dirs = dirs.len();

    // the starting nodes.
    let start_nodes = map.iter().filter(|(k,_)| -> bool {
        (*k & 0xFF) == 'A' as usize
    }).map(|(k,_)| -> usize { *k }).collect::<Vec<usize>>();

    // store nodes as current location and steps taken to get there. 
    let mut curr_nodes: Vec<(usize, usize)> = Vec::new();
    for node in start_nodes.iter() {
        curr_nodes.push((*node, 0));
    }


    // advance each node to the end. 
    for i in 0..curr_nodes.len() {
        let (node, steps) = curr_nodes[i];
        curr_nodes[i] = steps_to_end((node, steps), &dirs, &map);
    }

    // cache the node and steps to get to the next ending Z.
    // nodes + steps -> (node, steps % dirs.len())
    let mut cache: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // until all nodes are at the end with the same number of steps, advance each node to next z.
    let mut curr_max_steps = curr_nodes.iter().map(|(_,steps)| -> usize { *steps }).max().unwrap();
    while !curr_nodes.iter().all(|(_,steps)| -> bool { *steps == curr_max_steps }) {
        for i in 0..curr_nodes.len() {
            let (node, steps) = curr_nodes[i];
            let dir_loc = steps % n_dirs;
            if steps != curr_max_steps {
                if cache.contains_key(&(node, dir_loc)) {
                    let (new_node, advanced_by) = cache.get(&(node, dir_loc)).unwrap();
                    curr_nodes[i] = (*new_node, steps + advanced_by);
                } else {
                    let (new_node, advanced_by) = steps_to_end((node, steps), &dirs, &map);
                    curr_nodes[i] = (new_node, steps + advanced_by);
                    cache.insert((node, dir_loc), (new_node, advanced_by));
                }
            }
        }
        curr_max_steps = curr_nodes.iter().map(|(_,steps)| -> usize { *steps }).max().unwrap();
    }

    println!("final config:");
    for (node, steps) in curr_nodes.iter() {
        println!("node = {}, steps = {}", id_to_str(*node), steps);
    }

    return curr_max_steps;
}

fn part1(filename: &str) -> usize {
    let (dirs, map) = parse_input(filename);

    let start = node_to_id("AAA");
    let end = node_to_id("ZZZ");

    let mut curr = start;
    let mut curr_idx = 0;
    let n = dirs.len();

    let mut n_steps = 0;
    while curr != end { 
        curr = map.get(&curr).unwrap()[dirs[curr_idx]];
        n_steps += 1;
        curr_idx = (curr_idx + 1) % n;
    }

    return n_steps;
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day08_sample.txt"), 2);
        assert_eq!(part1("files/day08_sample2.txt"), 6);
        assert_eq!(part1("files/day08_input.txt"), 13771);
        assert_eq!(part2("files/day08_sample3.txt"), 6);
        assert_eq!(part2_withlcm("files/day08_input.txt"), 13129439557681);
    }
}