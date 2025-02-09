use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut priority_score: i32 = 0;
    for line in contents.lines() {
        let halves = line.split_at(line.len() / 2);
        let first_half = to_priority_set(halves.0.to_string());
        let second_half = to_priority_set(halves.1.to_string());
        let mut intersection = first_half.intersection(&second_half);

        priority_score += intersection.into_iter().sum::<i32>();
    }

    return priority_score;
}

fn part2(contents: String) -> i32 {
    let mut priority_score: i32 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let j = i + 1;
        let k = i + 2;

        let first = to_priority_set(lines[i].to_string());
        let second = to_priority_set(lines[j].to_string());
        let third = to_priority_set(lines[k].to_string());

        let mut intersection: HashSet<_> = first.intersection(&second).cloned().collect();
        intersection = intersection.intersection(&third).cloned().collect();

        priority_score += intersection.into_iter().sum::<i32>();

    }

    return priority_score;
}

fn to_priority_set(half: String) -> HashSet<i32> {
    let mut items: HashSet<i32> = HashSet::new();
    for c in half.chars() {
        if 'a' <= c && c <= 'z' {
            items.insert(c as i32 - 'a' as i32 + 1);
        } else {
            items.insert(c as i32 - 'A' as i32 + 27);
        }
    }

    return items;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 157);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 70);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "3".to_string();
	
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
        "\nPart 1:\nPriority score: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPriority score: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}