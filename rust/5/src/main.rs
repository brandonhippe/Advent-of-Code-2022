use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut doing_moves = false;

    for line in contents.lines() {
        if doing_moves {
            let words: Vec<&str> = line.split(" ").collect::<Vec<_>>();
            let num = words[1].parse::<i32>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();

            for _i in 0..num {
                let f_c = &stacks[from - 1 as usize].pop_back().unwrap();
                stacks[to - 1 as usize].push_back(f_c.to_string());
            }
        } else {
            if line.len() == 0 {
                doing_moves = true;
                continue;
            }

            for i in (1..line.len()).step_by(4) {
                if stacks.len() == (i - 1) / 4 {
                    stacks.push(VecDeque::new());
                }

                let c: char = line.chars().nth(i).expect("Couldn't get char");


                if 'A' <= c && c <= 'Z' {
                    stacks[(i - 1) / 4].push_front(c.to_string());
                }
            }
        }
    }

    return stacks
        .iter_mut()
        .map(|s| s.pop_back().unwrap())
        .collect::<String>();
}

fn part2(contents: String) -> String {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut doing_moves = false;

    for line in contents.lines() {
        if doing_moves {
            let words: Vec<&str> = line.split(" ").collect::<Vec<_>>();
            let num = words[1].parse::<i32>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();

            let mut moving: VecDeque<String> = VecDeque::new();
            for _i in 0..num {
                moving.push_front(stacks[(from - 1) as usize].pop_back().unwrap());
            }

            stacks[(to - 1) as usize].append(&mut moving);
        } else {
            if line.len() == 0 {
                doing_moves = true;
                continue;
            }

            for i in (1..line.len()).step_by(4) {
                if stacks.len() == (i - 1) / 4 {
                    stacks.push(VecDeque::new());
                }

                let c: char = line.chars().nth(i).expect("Couldn't get char");

                if 'A' <= c && c <= 'Z' {
                    stacks[(i - 1) / 4].push_front(c.to_string());
                }
            }
        }
    }

    return stacks
        .iter_mut()
        .map(|s| s.pop_back().unwrap())
        .collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "CMZ");
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "MCD");
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "5".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
	} else if root.ends_with(format!("rust_{}_{}", year, day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nCrates on top: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCrates on top: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}