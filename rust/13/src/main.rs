use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let lines: Vec<String> = contents
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| line.to_string())
        .collect();

    let mut ix_sums: i32 = 0;
    for i in (0..lines.len()).step_by(2) {
        let left: String = lines[i].clone();
        let right: String = lines[i + 1].clone();

        if compare_strings(left.clone(), right.clone()) == -1 {
            ix_sums += i as i32 / 2 + 1;
        }
    }

    return ix_sums;
}

fn part2(contents: String) -> i32 {
    let divider_packets: Vec<String> = vec!["[[2]]".to_string(), "[[6]]".to_string()];
    let mut lines: Vec<String> = contents
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| line.to_string())
        .collect();

    lines.append(&mut divider_packets.clone());
    lines.sort_by(|a, b| compare_strings(a.clone(), b.clone()).cmp(&0));


    return lines
        .iter()
        .enumerate()
        .filter(|(_, line)| divider_packets.contains(line))
        .map(|(ix, _)| ix as i32 + 1)
        .product::<i32>();
}

fn compare_strings(left: String, right: String) -> i32 {
    if left.len() == 0 || right.len() == 0 {
        return if left.len() == right.len() {
            0
        } else if left.len() == 0 {
            -1
        } else {
            1
        };
    }

    let mut left_ix = 0;
    let mut right_ix = 0;

    if !left.contains('[') && !right.contains('[') {
        let left_nums: Vec<i32> = left
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let right_nums: Vec<i32> = right
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        while left_ix < left_nums.len() && right_ix < right_nums.len() {
            if left_nums[left_ix] < right_nums[right_ix] {
                return -1;
            } else if left_nums[left_ix] > right_nums[right_ix] {
                return 1;
            }

            left_ix += 1;
            right_ix += 1;
        }

        return if left_nums.len() == right_nums.len() {
            0
        } else if left_nums.len() < right_nums.len() {
            -1
        } else {
            1
        };
    }

    loop {
        if left_ix > left.len() || right_ix > right.len() {
            return if left_ix > left.len() && right_ix > right.len() {
                0
            } else if left_ix > left.len() {
                -1
            } else {
                1
            };
        }

        let left_first: String = get_first_string(&left, &mut left_ix);
        let right_first: String = get_first_string(&right, &mut right_ix);

        let compared: i32;
        if left_first.starts_with('[') && right_first.starts_with('[') {
            compared = compare_strings(
                left_first[1..left_first.len() - 1].to_string(),
                right_first[1..right_first.len() - 1].to_string(),
            );
        } else if left_first.starts_with('[') {
            compared = compare_strings(left_first, "[".to_owned() + &right_first + "]");
        } else if right_first.starts_with('[') {
            compared = compare_strings("[".to_owned() + &left_first + "]", right_first);
        } else {
            compared = compare_strings(left_first, right_first);
        }

        if compared != 0 {
            return compared;
        }
    }
}

fn get_first_string(contents: &str, ix: &mut usize) -> String {
    let mut first_string: String = "".to_string();

    if contents[*ix..*ix + 1] == "[".to_string() {
        let mut bracket_count = -1;
        while bracket_count != 0 {
            if bracket_count == -1 {
                bracket_count = 0;
            }

            if contents[*ix..*ix + 1] == "[".to_string() {
                bracket_count += 1;
            } else if contents[*ix..*ix + 1] == "]".to_string() {
                bracket_count -= 1;
            }

            first_string.push_str(&contents[*ix..*ix + 1]);
            *ix += 1;
        }
        *ix += 1;
    } else {
        while *ix < contents.len() && contents[*ix..*ix + 1] != ",".to_string() {
            first_string.push_str(&contents[*ix..*ix + 1]);
            *ix += 1;
        }

        *ix += 1;
    }

    return first_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 13);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 140);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "13".to_string();
	
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
        "\nPart 1:\nSum of packet indexes in correct order: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProduct of divider packet locations: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}