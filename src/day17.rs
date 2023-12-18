use crate::common::get_lines;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap; 

type Direction = (isize, isize);
type Position = (usize, usize);
type Map = Vec<Vec<usize>>;

// Each cell in the map can be visited in many different ways and the way in
// which it is visited will affect the "next" branches from that cell.  This
// comes from the fact that we have a limitation on "straight" paths (need len)
// and we can't go backwards (thus need dir)
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Node { 
    pos: Position,
    dir: Direction,
    len: usize,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Node,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on cost.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.pos.cmp(&other.node.pos))
            .then_with(|| self.node.dir.cmp(&other.node.dir))
            .then_with(|| self.node.len.cmp(&other.node.len))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_map(filename: &str) -> Map {
    let lines = get_lines(filename);
    let mut map = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        map.push(row);
    }
    map
}

fn solve(map: &Map, min_len: usize, max_len: usize) -> usize { 
    let start = (0, 0);
    let end = (map.len() - 1, map[0].len() - 1);
    let mut heap = BinaryHeap::new();

    // store min cost to node and it's previous position.
    let mut costs: HashMap<Node, usize> = HashMap::new();

    // start from top left and init to either go down or right.
    for dir in [(0, 1), (1, 0)] { 
        let node = Node { pos: start, dir: dir, len: 0 };
        costs.insert(node, 0);
        heap.push(State { cost: 0, node: node });
    }

    while let Some(curr) = heap.pop() {
        // can't stop unless we have min_len moves.
        if curr.node.pos == end && curr.node.len >= min_len {
            return curr.cost;
        }

        // already found a better path.
        if let Some(cost) = costs.get(&curr.node) {
            if curr.cost > *cost {
                continue;
            }
        }

        let possible_moves: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let next_positions = 
            possible_moves
            .into_iter()
            .filter(|dir| { 
                // prevent going back.
                *dir != (-curr.node.dir.0, -curr.node.dir.1)
            })
            .filter(|dir| { 
                // *must* go straight if less than min.
                *dir == curr.node.dir || curr.node.len >= min_len
            })
            .filter(|dir| { 
                // prevent going straight for more than max.
                *dir != curr.node.dir || curr.node.len < max_len
            })
            .map(|dir| { 
                // convert to positions.
                let r = curr.node.pos.0 as isize + dir.0;
                let c = curr.node.pos.1 as isize + dir.1;
                (r, c)
            })
            .filter(|pos| {
                // make sure it's within bounds.
                (pos.0 >= 0 && pos.0 < map.len() as isize) && (pos.1 >= 0 && pos.1 < map[0].len() as isize)
            });

        for next_pos in next_positions {
            let new_cost = curr.cost + map[next_pos.0 as usize][next_pos.1 as usize] as usize;
            let new_dir = (next_pos.0 as isize - curr.node.pos.0 as isize, next_pos.1 as isize - curr.node.pos.1 as isize);
            let new_node = Node { 
                pos: (next_pos.0 as usize, next_pos.1 as usize),
                dir: new_dir,
                len: if curr.node.dir == new_dir { curr.node.len + 1 } else { 1 },
            };

            // we can't record optimal path if we haven't reached min_len straight moves.
            if new_node.len < min_len {
                heap.push(State{ 
                    cost: new_cost,
                    node: new_node
                });
            } else if new_cost < *costs.get(&new_node).unwrap_or(&usize::MAX) {
                costs.entry(new_node)
                    .and_modify(|c| *c = new_cost)
                    .or_insert(new_cost);
                heap.push(State{ 
                    cost: new_cost,
                    node: new_node
                });
            }
        }
    }
    println!("no path found");
    return usize::MAX;
}

fn part1(filename: &str) -> usize { 
    let map = parse_map(filename);
    solve(&map, 1, 3)
}

fn part2(filename: &str) -> usize { 
    let map = parse_map(filename);
    solve(&map, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1("files/day17_sample.txt"), 102);
        assert_eq!(part1("files/day17_input.txt"), 755);
        assert_eq!(part2("files/day17_sample.txt"), 94);
        assert_eq!(part2("files/day17_sample2.txt"), 71);
        assert_eq!(part2("files/day17_input.txt"), 881);
    }
}