use relative_path::RelativePath;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut levels: Vec<Vec<i32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (y, line) in contents.lines().enumerate() {
        let mut new_vec: Vec<i32> = Vec::new();
        for (x, num) in line.chars().enumerate() {
            if num == 'S' {
                new_vec.push('a' as i32);
                start = (x, y);
            } else if num == 'E' {
                new_vec.push('z' as i32);
                end = (x, y);
            } else {
                new_vec.push(num as i32);
            }
        }

        levels.push(new_vec);
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut open: VecDeque<(usize, usize)> = vec![start].into();
    let mut steps = 0;

    loop {
        let mut new_open: VecDeque<(usize, usize)> = VecDeque::new();


        while !open.is_empty() {
            let current = open.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }

            if current == end {
                return steps;
            }

            let current_height = levels[current.1][current.0];
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            if current.0 > 0 && levels[current.1][current.0 - 1] <= current_height + 1 {
                neighbors.push((current.0 - 1, current.1));
            }
            if current.0 < levels[0].len() - 1
                && levels[current.1][current.0 + 1] <= current_height + 1
            {
                neighbors.push((current.0 + 1, current.1));
            }
            if current.1 > 0 && levels[current.1 - 1][current.0] <= current_height + 1 {
                neighbors.push((current.0, current.1 - 1));
            }
            if current.1 < levels.len() - 1
                && levels[current.1 + 1][current.0] <= current_height + 1
            {
                neighbors.push((current.0, current.1 + 1));
            }

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    new_open.push_back(neighbor);
                }
            }

            visited.insert(current);
        }

        open = new_open;
        steps += 1;
    }
}

fn part2(contents: String) -> i32 {
    let mut levels: Vec<Vec<i32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);

    for (y, line) in contents.lines().enumerate() {
        let mut new_vec: Vec<i32> = Vec::new();
        for (x, num) in line.chars().enumerate() {
            if num == 'S' {
                new_vec.push(-('a' as i32));
            } else if num == 'E' {
                new_vec.push(-('z' as i32));
                start = (x, y);
            } else {
                new_vec.push(-(num as i32));
            }
        }

        levels.push(new_vec);
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut open: VecDeque<(usize, usize)> = vec![start].into();
    let mut steps = 0;

    loop {
        let mut new_open: VecDeque<(usize, usize)> = VecDeque::new();

        while !open.is_empty() {
            let current = open.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }

            if levels[current.1][current.0] == -('a' as i32) {
                return steps;
            }

            let current_height = levels[current.1][current.0];
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            if current.0 > 0 && levels[current.1][current.0 - 1] <= current_height + 1 {
                neighbors.push((current.0 - 1, current.1));
            }
            if current.0 < levels[0].len() - 1
                && levels[current.1][current.0 + 1] <= current_height + 1
            {
                neighbors.push((current.0 + 1, current.1));
            }
            if current.1 > 0 && levels[current.1 - 1][current.0] <= current_height + 1 {
                neighbors.push((current.0, current.1 - 1));
            }
            if current.1 < levels.len() - 1
                && levels[current.1 + 1][current.0] <= current_height + 1
            {
                neighbors.push((current.0, current.1 + 1));
            }

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    new_open.push_back(neighbor);
                }
            }

            visited.insert(current);
        }

        open = new_open;
        steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 31);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 29);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "12".to_string();
	
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
        "\nPart 1:\nShortest path to top: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest path to bottom: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}