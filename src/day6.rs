use num::Float;
use crate::common::get_lines;
use std::iter::zip;

fn find_roots(time: usize, dist: usize) -> (usize, usize) {
    // quadratic formula: 
    let a: f64 = -1.0;
    let b: f64 = time as f64;
    let c: f64 = -(dist as f64);

    let determinant = b.powi(2) - 4.0 * a * c;
    if (determinant as i32) > 0 {
        let mut roots = (
            Float::ceil(((-b + determinant.sqrt()) / (2.0 * a))) as usize,
            Float::floor(((-b - determinant.sqrt()) / (2.0 * a))) as usize
        );

        // adjust in case we find exact roots.
        let vals = ((time - roots.0) * roots.0, (time - roots.1) * roots.1);
        if vals.0 <= dist { 
            roots.0 += 1;
        } 
        if vals.1 <= dist {
            roots.1 -= 1;
        }

        return roots;
    } else { 
        panic!("invalid roots found");
    }
}

fn part1(filename: &str) -> usize {
    let lines = get_lines(filename);

    let mut line_iter = lines.iter();

    let mut time_line= line_iter.next().unwrap().split_whitespace();
    time_line.next();
    let times: Vec<usize> = time_line.map(|x| x.parse::<usize>().unwrap()).collect();

    let mut dist_line = line_iter.next().unwrap().split_whitespace();
    dist_line.next();
    let dists: Vec<usize> = dist_line.map(|x| x.parse::<usize>().unwrap()).collect();

    let roots = zip(times.iter(), dists.iter()).map(|(t, d)| find_roots(*t, *d));
    roots.map(|(a, b)| b - a + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roots() {
        assert_eq!(find_roots(7, 9), (2, 5));
        assert_eq!(find_roots(15, 40), (4, 11));
        assert_eq!(find_roots(30, 200), (11, 19));
    }

    #[test]
    fn test() {
        assert_eq!(part1("files/day06_sample.txt"), 288);
        assert_eq!(part1("files/day06_input.txt"), 1660968);

        // do part 2 manually instead of worrying about parsing the file.
        { 
            // sample
            let roots = find_roots(71530, 940200);
            assert_eq!(roots.1 - roots.0 + 1, 71503);
        }
        { 
            // input
            let roots = find_roots(47986698, 400121310111540);
            assert_eq!(roots.1 - roots.0 + 1, 71503);
        }
    }
}
