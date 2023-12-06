use crate::common::get_lines;

#[derive(Debug)]
struct MapEntry {
    src_start: usize, 
    dst_start: usize, 
    size: usize
}

// intervals are defined as a tuple by starting value and size

// return true if val is inside interval a
fn is_inside(a: (usize, usize), val: usize) -> bool {
    return val >= a.0 && val < a.0 + a.1;
}

// if interval a is completely inside interval b
fn is_inside_interval(a: (usize, usize), b: (usize, usize)) -> bool {
    let a_start = a.0;
    let a_end = a.0 + a.1 - 1;
    return is_inside(b, a_start) && is_inside(b, a_end);
}

// return true if the intervals overlap
fn is_overlapping(a: (usize, usize), b: (usize, usize)) -> bool {
    let a_start = a.0;
    let a_end = a.0 + a.1 - 1;
    let b_start = b.0;
    let b_end = b.0 + b.1 - 1;
    return is_inside(b, a_start) || is_inside(b, a_end) || is_inside(a, b_start) || is_inside(a, b_end);
}

// split the interval if they are overlapping into two intervals, one
// that is inside the other, and one that is outside the other.
fn split_interval(a: (usize, usize), b: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let a_start = a.0;
    let a_end = a.0 + a.1 - 1;
    let b_start = b.0;
    let b_end = b.0 + b.1 - 1;
    if is_inside(b, a_start) {
        // a_start is inside b
        let a1 = (a_start, b_end - a_start + 1);
        let a2 = (b_end + 1, a_end - b_end);
        return (a1, a2);
    } else if is_inside(b, a_end) {
        // a_end is inside b
        let a1 = (a_start, b_start - a_start);
        let a2 = (b_start, a_end - b_start + 1);
        return (a1, a2);
    } else {
        panic!("split_interval: intervals do not overlap");
    }
}

fn map_to_dst(src: usize, entry: &MapEntry) -> Option<usize> {
    let src_end = entry.src_start + entry.size;
    if src >= entry.src_start && src < src_end {
        let offset = src - entry.src_start;
        let dst = entry.dst_start + offset;
        //println!("{} -> {}", src, dst);
        return Some(dst);
    }
    return None
}

fn src_to_dst(src: usize, map: &Vec<MapEntry>) -> usize {
    for entry in map {
        if let Some(dst) = map_to_dst(src, entry) {
            return dst;
        }
    }
    return src
}

// break up the given interval into many sub-intervals such that each resulting
// interval is contained entirely within a MapEntry or completely outside of 
// all MapEntries.
fn split_interval_in_map(a: (usize, usize), map: &Vec<MapEntry>) -> Vec<(usize, usize)> {
    let mut splits: Vec<(usize, usize)> = Vec::new();
    for entry in map {
        if is_inside_interval(a, (entry.src_start, entry.size)) {
            return vec![a];
        } else if is_inside_interval((entry.src_start, entry.size), a) {
            let left = (a.0, entry.src_start - a.0);
            let center = (entry.src_start, entry.size);
            let right = (entry.src_start + entry.size, a.0 + a.1 - entry.src_start - entry.size);
            if left.1 > 0 {
                let left_splits = split_interval_in_map(left, map);
                splits.extend(left_splits);
            }
            if center.1 > 0 {
                splits.push(center);
            }
            if right.1 > 0 {
                let right_splits = split_interval_in_map(right, map);
                splits.extend(right_splits);
            }
            break;
        } else if is_overlapping(a, (entry.src_start, entry.size)) {
            let (left, right) = split_interval(a, (entry.src_start, entry.size));
            let left_splits = split_interval_in_map(left, map);
            let right_splits = split_interval_in_map(right, map);
            splits.extend(left_splits);
            splits.extend(right_splits);
            break;
        }
    }
    // no overlapping intervals with entries in map, so we can just pass a along
    if splits.len() == 0 {
        splits.push(a);
    }
    return splits;
}

fn ranged_src_to_dst(src: (usize, usize), map: &Vec<MapEntry>) -> (usize, usize) {
    let dst = src;
    for entry in map {
        if is_inside_interval(src, (entry.src_start, entry.size)) {
            let offset = src.0 - entry.src_start;
            let dst = entry.dst_start + offset;
            return (dst, src.1);
        }
    }
    return dst;
}

fn part1(filename: &str) -> usize {
    let lines = get_lines(filename);

    let mut seeds: Vec<usize> = Vec::new();

    let mut line_iter = lines.iter();

    let seed_line = line_iter.next().unwrap();
    let seed_strings = seed_line.split(": ").last().unwrap().split_whitespace();
    for seed_string in seed_strings {
        seeds.push(seed_string.parse::<usize>().unwrap());
    }

    let mut locations = seeds.clone();

    line_iter.next(); // skip empty line
    line_iter.next(); // skip map header

    let mut map: Vec<MapEntry> = Vec::new();
    while let Some(curr_line) = line_iter.next() {
        if curr_line.contains("map") {
            continue;
        } else if curr_line == "" {
            // end the map, find memberships.
            for i in 0..locations.len() {
                let src = locations[i];
                let dst = src_to_dst(src, &map);
                locations[i] = dst;
            }
            map.clear();
        } else {
            let mut parts = curr_line.split_whitespace();
            let dst = parts.next().unwrap().parse::<usize>().unwrap();
            let src = parts.next().unwrap().parse::<usize>().unwrap();
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            map.push(MapEntry{src_start: src, dst_start: dst, size: size});
        }
    }
    println!("{:?}", locations);
    return *locations.iter().min().unwrap();
}

fn part2(filename: &str) -> usize {
    let lines = get_lines(filename);

    let mut seeds: Vec<usize> = Vec::new();

    let mut line_iter = lines.iter();

    let seed_line = line_iter.next().unwrap();
    let seed_strings = seed_line.split(": ").last().unwrap().split_whitespace();
    for seed_string in seed_strings {
        seeds.push(seed_string.parse::<usize>().unwrap());
    }

    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();

    let mut seed_iter = seeds.iter();
    while let Some(val) = seed_iter.next() {
        let seed = *val;
        let size = seed_iter.next().unwrap();
        seed_ranges.push((seed, *size));
    }

    println!("{:?}", seed_ranges);
    let mut locations = seed_ranges.clone();

    line_iter.next(); // skip empty line
    line_iter.next(); // skip map header

    let mut map: Vec<MapEntry> = Vec::new();
    while let Some(curr_line) = line_iter.next() {
        if curr_line.contains("map") {
            //println!("{}", curr_line);
            continue;
        } else if curr_line == "" {
            //println!("{:?}", map);
            // end the map, find memberships.
            let mut new_locations: Vec<(usize, usize)> = Vec::new();
            for i in 0..locations.len() {
                let src = locations[i];
                let splits = split_interval_in_map(src, &map);
                //println!("splits = {:?} -> {:?}", src, splits);
                for split in splits {
                    let dst = ranged_src_to_dst(split, &map);
                    //println!("{:?} -> {:?}", split, dst);
                    new_locations.push(dst);
                }
            }
            locations = new_locations;
            //println!("new locations: {:?}", locations);
            map.clear();
        } else {
            let mut parts = curr_line.split_whitespace();
            let dst = parts.next().unwrap().parse::<usize>().unwrap();
            let src = parts.next().unwrap().parse::<usize>().unwrap();
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            map.push(MapEntry{src_start: src, dst_start: dst, size: size});
        }
    }
    println!("{:?}", locations);
    return locations.iter().map(|x| x.0).min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_dst() {
        assert_eq!(map_to_dst(50, &MapEntry{src_start: 50, dst_start: 98, size: 2}), Some(98));
        assert_eq!(map_to_dst(49, &MapEntry{src_start: 50, dst_start: 98, size: 2}), None);
        assert_eq!(map_to_dst(52, &MapEntry{src_start: 50, dst_start: 98, size: 2}), None);
        assert_eq!(map_to_dst(51, &MapEntry{src_start: 50, dst_start: 98, size: 2}), Some(99));

        assert_eq!(src_to_dst(50, &vec![MapEntry{src_start: 50, dst_start: 98, size: 2}, 
                                        MapEntry{src_start: 52, dst_start: 50, size: 48}]), 98);
        assert_eq!(src_to_dst(10, &vec![MapEntry{src_start: 50, dst_start: 98, size: 2}, 
                                        MapEntry{src_start: 52, dst_start: 50, size: 48}]), 10);
    }

    #[test]
    fn test_ranged_map() {
        assert_eq!(ranged_src_to_dst((52, 14), &vec![MapEntry{src_start: 50, dst_start: 98, size: 2}, 
                                                     MapEntry{src_start: 52, dst_start: 50, size: 48}]), (50, 14));
        assert_eq!(ranged_src_to_dst((10, 14), &vec![MapEntry{src_start: 50, dst_start: 98, size: 2}, 
                                                     MapEntry{src_start: 52, dst_start: 50, size: 48}]), (10, 14));
    }

    #[test]
    fn test_split_interval() {
        assert_eq!(split_interval((1, 10), (5, 10)), ((1, 4), (5, 6)));
        assert_eq!(split_interval((5, 10), (1, 8)), ((5, 4), (9, 6)));
        assert_eq!(split_interval_in_map((57, 13), &vec![MapEntry { src_start: 53,
        dst_start: 49, size: 8 }, MapEntry { src_start: 11, dst_start: 0, size:
        42 }, MapEntry { src_start: 0, dst_start: 42, size: 7 }, MapEntry {
        src_start: 7, dst_start: 57, size: 4 }]), vec![(57, 4), (61, 9)]);

        assert_eq!(split_interval_in_map((74, 14),
        &vec![MapEntry { src_start: 77, dst_start: 45, size: 23 }, MapEntry {
        src_start: 45, dst_start: 81, size: 19 }, MapEntry { src_start: 64,
        dst_start: 68, size: 13 }]), vec![(74, 3), (77,11)]);
    }

    #[test]
    fn test() {
        assert_eq!(part1("files/day05_sample.txt"), 35);
        assert_eq!(part1("files/day05_input.txt"), 910845529);
        assert_eq!(part2("files/day05_sample.txt"), 46);
        assert_eq!(part2("files/day05_input.txt"), 77435348);
    }
}
