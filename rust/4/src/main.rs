use itertools::Itertools;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut contained = 0;
    for line in contents.lines() {
        let (g1, g2) = line.split(",").collect_tuple().unwrap();

        let (g1_min, g1_max) = g1
            .to_string()
            .split("-")
            .map(|num| num.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (g2_min, g2_max) = g2
            .to_string()
            .split("-")
            .map(|num| num.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if (g1_min <= g2_min && g2_max <= g1_max) || (g2_min <= g1_min && g1_max <= g2_max) {
            contained += 1;
        }
    }

    return contained;
}

fn part2(contents: String) -> i32 {
    let mut overlap = 0;
    for line in contents.lines() {

        let (g1, g2) = line.split(",").collect_tuple().unwrap();

        let (g1_min, g1_max) = g1
            .to_string()
            .split("-")
            .map(|num| num.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let (g2_min, g2_max) = g2
            .to_string()
            .split("-")
            .map(|num| num.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if (g1_min <= g2_min && g2_min <= g1_max) || (g2_min <= g1_min && g1_min <= g2_max) {
            overlap += 1;
        }
    }

    return overlap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 2);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "4".to_string();
	
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
        "\nPart 1:\nFully contained ranges: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nOverlapping ranges: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}