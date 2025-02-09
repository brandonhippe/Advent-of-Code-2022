use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let nums: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut visible: HashSet<(i32, i32)> = HashSet::new();

    for y in 1..nums.len() - 1 {
        let mut max_visible = -1;
        for x in 0..nums[y].len() {
            let n = nums[y][x];
            if n > max_visible {
                max_visible = n;
                visible.insert((x as i32, y as i32));
            }
        }
    }

    for y in 1..nums.len() - 1 {
        let mut max_visible = -1;
        for x in (0..nums[y].len()).rev() {
            let n = nums[y][x];
            if n > max_visible {

                max_visible = n;
                visible.insert((x as i32, y as i32));
            }
        }
    }

    for x in 1..nums[0].len() - 1 {
        let mut max_visible = -1;
        for y in 0..nums.len() {
            let n = nums[y][x];
            if n > max_visible {
                max_visible = n;
                visible.insert((x as i32, y as i32));
            }
        }
    }

    for x in 1..nums[0].len() - 1 {
        let mut max_visible = -1;
        for y in (0..nums.len()).rev() {
            let n = nums[y][x];
            if n > max_visible {
                max_visible = n;
                visible.insert((x as i32, y as i32));
            }
        }
    }

    return visible.len() as i32 + 4;
}

fn part2(contents: String) -> i32 {
    let nums: HashMap<(i32, i32), i32> =
        HashMap::from_iter(contents.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, num)| {
                    (
                        (x as i32, y as i32),
                        num.to_string().parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<((i32, i32), i32)>>()
        }));

    let max_x = nums.keys().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = nums.keys().max_by_key(|(_, y)| y).unwrap().1;

    let mut max_score = 0;
    for ((x, y), n) in nums.iter() {
        let mut score = 1;
        let mut viewing = 0;
        for i in x + 1..=max_x {
            viewing += 1;

            if nums.get(&(i, *y)).unwrap() >= n {
                break;
            }
        }

        score *= viewing;
        viewing = 0;

        for i in (0..=x - 1).rev() {
            viewing += 1;

            if nums.get(&(i, *y)).unwrap() >= n {
                break;
            }
        }

        score *= viewing;
        viewing = 0;

        for i in y + 1..=max_y {
            viewing += 1;

            if nums.get(&(*x, i)).unwrap() >= n {
                break;
            }
        }

        score *= viewing;
        viewing = 0;

        for i in (0..=y - 1).rev() {
            viewing += 1;

            if nums.get(&(*x, i)).unwrap() >= n {
                break;
            }
        }

        score *= viewing;
        max_score = std::cmp::max(max_score, score);
    }

    return max_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 21);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 8)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "8".to_string();
	
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
        "\nPart 1:\nVisible tree: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHighest score: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}