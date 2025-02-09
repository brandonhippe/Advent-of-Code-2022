use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String, row: i32) -> i32 {
    let num_re = Regex::new(r"(-?\d+)").unwrap();
    let sensor_dists: HashMap<(i32, i32), i32> = HashMap::from_iter(contents.lines().map(|line| {
        let nums: Vec<i32> = num_re
            .captures_iter(line)
            .map(|x| x[0].parse::<i32>().unwrap())
            .collect();
        (
            (nums[0], nums[1]),
            manhattan_distance(nums[0], nums[1], nums[2], nums[3]),
        )
    }));

    let beacons: HashSet<(i32, i32)> = HashSet::from_iter(contents.lines().map(|line| {
        let nums: Vec<i32> = num_re
            .captures_iter(line)
            .map(|x| x[0].parse::<i32>().unwrap())
            .collect();
        (nums[2], nums[3])
    }));

    let mut row_ranges: Vec<(i32, i32)> = Vec::from(
        sensor_dists
            .iter()
            .filter(|((_, y), d)| **d >= (y - row).abs())
            .map(|((x, y), d)| (x - (d - (y - row).abs()), x + (d - (y - row).abs())))
            .collect::<Vec<(i32, i32)>>(),

    );
    row_ranges.sort();

    let mut changed: bool = true;
    while changed {
        changed = false;
        for i in (1..row_ranges.len()).rev() {
            if row_ranges[i - 1].1 >= row_ranges[i].0 {
                row_ranges[i - 1].1 = std::cmp::max(row_ranges[i - 1].1, row_ranges[i].1);
                row_ranges.remove(i);
                changed = true;
            }
        }
    }

    return row_ranges.iter().map(|(x1, x2)| x2 - x1 + 1).sum::<i32>()
        - beacons
            .iter()
            .filter(|(x, y)| *y == row && *x >= row_ranges[0].0 && *x <= row_ranges[0].1)
            .count() as i32;
}

fn part2(contents: String, half_dim: i32) -> i64 {
    let num_re = Regex::new(r"(-?\d+)").unwrap();
    let sensor_dists: HashMap<(i32, i32), i32> = HashMap::from_iter(contents.lines().map(|line| {
        let nums: Vec<i32> = num_re
            .captures_iter(line)
            .map(|x| x[0].parse::<i32>().unwrap())
            .collect();
        (
            (nums[0], nums[1]),
            manhattan_distance(nums[0], nums[1], nums[2], nums[3]),
        )
    }));

    let max_coord: i32 = half_dim * 2;

    for row in 0..max_coord {
        let mut row_ranges: Vec<(i32, i32)> = Vec::from(
            sensor_dists
                .iter()
                .filter(|((_, y), d)| **d >= (y - row).abs())
                .map(|((x, y), d)| (x - (d - (y - row).abs()), x + (d - (y - row).abs())))
                .collect::<Vec<(i32, i32)>>(),
        );
        row_ranges.sort();

        let mut changed: bool = true;
        while changed {
            changed = false;
            for i in (1..row_ranges.len()).rev() {
                if row_ranges[i - 1].1 >= row_ranges[i].0 {
                    row_ranges[i - 1].1 = std::cmp::max(row_ranges[i - 1].1, row_ranges[i].1);
                    row_ranges.remove(i);
                    changed = true;
                }
            }
        }

        if row_ranges.len() > 1 {
            return row_ranges
                .windows(2)
                .filter(|x| x[0].1 + 1 < x[1].0)
                .map(|x| x[0].1 + 1)
                .sum::<i32>() as i64
                * 4000000
                + row as i64;
        }
    }

    return -1;
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 10), 26);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 10), 56000011);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "15".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};
    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nPositions at y = 2000000 that can't contain a beacon: {}\nRan in {:.5?}",
        part1(contents.clone(), 2000000),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTuning frequency of distress beacon: {}\nRan in {:.5?}",
        part2(contents.clone(), 2000000),
        part2_timer.elapsed()
    );
}