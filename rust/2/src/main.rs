use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let games: Vec<(i32, i32)> = contents
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap() as i32 - 'A' as i32,
                line.chars().nth(2).unwrap() as i32 - 'X' as i32,
            )
        })
        .collect();

    return games
        .iter()
        .map(|(other, you)| ((you - other + 4) % 3) * 3 + (you + 1))
        .sum();
}

fn part2(contents: String) -> i32 {
    let games: Vec<(i32, i32)> = contents
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap() as i32 - 'A' as i32,
                line.chars().nth(2).unwrap() as i32 - 'X' as i32,
            )
        })
        .collect();

    return games
        .iter()

        .map(|(other, res)| {
            let you = (other + res + 2) % 3;
            ((you - other + 4) % 3) * 3 + (you + 1)
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 15);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 12);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "2".to_string();
	
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
        "\nPart 1:\nScore: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nScore: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}