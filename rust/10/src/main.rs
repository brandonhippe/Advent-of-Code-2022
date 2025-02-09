use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut x_reg: i32 = 1;
    let mut signal_strength: i32 = 0;
    let mut cycle: i32 = 1;

    for line in contents.lines() {
        if cycle % 40 == 20 {
            signal_strength += x_reg * cycle;
        }

        let instruction = line.split_whitespace().nth(0).unwrap();

        match instruction {
            "addx" => {
                cycle += 1;

                if cycle % 40 == 20 {
                    signal_strength += x_reg * cycle;
                }

                x_reg += line
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            "noop" => (),
            _ => panic!("Unknown instruction"),
        }


        cycle += 1
    }

    if cycle % 40 == 20 {
        signal_strength += x_reg * cycle;
    }

    return signal_strength;
}

fn part2(contents: String) -> String {
    let mut x_reg: i32 = 1;
    let mut cycle: i32 = 1;

    let mut output = "\n".to_string();
    for line in contents.lines() {
        output.push_str(if ((cycle - 1) % 40 - x_reg).abs() <= 1 {
            "█"
        } else {
            " "
        });
        if cycle % 40 == 0 {
            output.push_str("\n");
        }

        let instruction = line.split_whitespace().nth(0).unwrap();

        match instruction {
            "addx" => {
                cycle += 1;

                output.push_str(if ((cycle - 1) % 40 - x_reg).abs() <= 1 {
                    "█"
                } else {
                    " "
                });
                if cycle % 40 == 0 {
                    output.push_str("\n");
                }

                x_reg += line
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            "noop" => (),
            _ => panic!("Unknown instruction"),
        }

        cycle += 1;
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 13140);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "\n██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████     \n".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "10".to_string();
	
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
        "\nPart 1:\nSum of signal strengths: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMessage: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}