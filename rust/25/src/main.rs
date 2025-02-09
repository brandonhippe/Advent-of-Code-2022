use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut sum_val = contents
        .lines()
        .map(|line| {
            let mut v: i64 = 0;
            for (i, c) in line.chars().rev().enumerate() {
                match c {
                    '0' => {
                        v += (0) * 5_i64.pow(i as u32);
                    }
                    '1' => {
                        v += (1) * 5_i64.pow(i as u32);
                    }
                    '2' => {
                        v += (2) * 5_i64.pow(i as u32);
                    }
                    '=' => {
                        v += (-2) * 5_i64.pow(i as u32);
                    }
                    '-' => {
                        v += (-1) * 5_i64.pow(i as u32);
                    }
                    _ => panic!("Unexpected character"),
                }
            }

            v
        })
        .sum::<i64>();


    let mut result_string = "".to_string();
    while sum_val != 0 {
        let remainder = sum_val % 5;
        sum_val /= 5;
        match remainder {
            0 => {
                result_string.push('0');
            }
            1 => {
                result_string.push('1');
            }
            2 => {
                result_string.push('2');
            }
            3 => {
                result_string.push('=');
                sum_val += 1;
            }
            4 => {
                result_string.push('-');
                sum_val += 1;
            }
            _ => panic!("Unexpected remainder"),
        }
    }

    return result_string.chars().rev().collect::<String>();
}

fn part2(_contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "2=-1=0".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "25".to_string();
	
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
        "\nPart 1:\nSum of SNAFU numbers: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}