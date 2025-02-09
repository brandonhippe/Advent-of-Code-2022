use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let paths: Vec<Vec<(i32, i32)>> = contents
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|part| {
                    part.split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple()
                        .expect("Tuple creation failed")
                })
                .collect()
        })
        .collect();

    let mut area: HashSet<(i32, i32)> = HashSet::new();
    for path in paths {
        for i in 0..path.len() - 1 {
            let (x1, y1) = path[i].clone();
            let (x2, y2) = path[i + 1].clone();
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    area.insert((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    area.insert((x, y1));
                }

            }
        }
    }

    let max_y = area.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut past_pos: Vec<(i32, i32)> = vec![(500, 0)];
    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    while past_pos.len() != 0 {
        let (x, y) = past_pos.pop().unwrap();

        if y == max_y {
            return sand.len() as i32;
        }

        let mut new_pos: (i32, i32) = (x, y + 1);
        for dx in vec![0, -1, 1] {
            new_pos = (x + dx, y + 1);

            if !area.contains(&new_pos) && !sand.contains(&new_pos) {
                past_pos.push((x, y));
                past_pos.push(new_pos);
                break;
            }
        }

        if past_pos.len() == 0 || *past_pos.last().unwrap() != new_pos {
            sand.insert((x, y));
        }
    }

    return sand.len() as i32;
}

fn part2(contents: String) -> i32 {
    let paths: Vec<Vec<(i32, i32)>> = contents
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|part| {
                    part.split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple()
                        .expect("Tuple creation failed")
                })
                .collect()
        })
        .collect();

    let mut area: HashSet<(i32, i32)> = HashSet::new();
    for path in paths {
        for i in 0..path.len() - 1 {
            let (x1, y1) = path[i].clone();
            let (x2, y2) = path[i + 1].clone();
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    area.insert((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    area.insert((x, y1));
                }
            }
        }
    }

    let max_y = area.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut past_pos: Vec<(i32, i32)> = vec![(500, 0)];
    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    while past_pos.len() != 0 {
        let (x, y) = past_pos.pop().unwrap();

        let mut new_pos: (i32, i32) = (x, y + 1);
        if y != max_y {
            for dx in vec![0, -1, 1] {
                new_pos = (x + dx, y + 1);

                if !area.contains(&new_pos) && !sand.contains(&new_pos) {
                    past_pos.push((x, y));
                    past_pos.push(new_pos);
                    break;
                }
            }
        }

        if past_pos.len() == 0 || *past_pos.last().unwrap() != new_pos {
            sand.insert((x, y));
        }
    }

    return sand.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 24);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 93);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "14".to_string();
	
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
        "\nPart 1:\nUnits of sand: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nUnits of sand: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}