use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        let monkey_name: String = parts[0][..parts[0].len() - 1].to_string();

        if parts.len() == 2 {
            let num: i64 = parts[1].parse().unwrap();
            monkeys.insert(
                monkey_name,
                Monkey {
                    num: Some(num),
                    left: "".to_string(),
                    right: "".to_string(),
                    op: ' ',
                },
            );
        } else {
            let left: String = parts[1].to_string();
            let op: char = parts[2].chars().next().unwrap();
            let right: String = parts[3].to_string();

            monkeys.insert(
                monkey_name,
                Monkey {
                    num: None,
                    left: left,

                    right: right,
                    op: op,
                },
            );
        }
    }

    return monkeys_yell(&mut monkeys, "root");
}

fn part2(contents: String) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        let monkey_name: String = parts[0][..parts[0].len() - 1].to_string();

        if parts.len() == 2 {
            let num: i64 = parts[1].parse().unwrap();
            monkeys.insert(
                monkey_name,
                Monkey {
                    num: Some(num),
                    left: "".to_string(),
                    right: "".to_string(),
                    op: ' ',
                },
            );
        } else {
            let left: String = parts[1].to_string();
            let op: char = parts[2].chars().next().unwrap();
            let right: String = parts[3].to_string();

            monkeys.insert(
                monkey_name,
                Monkey {
                    num: None,
                    left: left,
                    right: right,
                    op: op,
                },
            );
        }
    }

    if find_humn(&monkeys, &monkeys["root"].left) {
        return humn_yell(
            &mut monkeys.clone(),
            &monkeys["root"].left,
            monkeys_yell(&mut monkeys.clone(), &monkeys["root"].right),
        );
    } else {
        return humn_yell(
            &mut monkeys.clone(),
            &monkeys["root"].right,
            monkeys_yell(&mut monkeys.clone(), &monkeys["root"].left),
        );
    }
}

fn monkeys_yell(monkeys: &mut HashMap<String, Monkey>, root_name: &str) -> i64 {
    while monkeys[root_name].num.is_none() {
        for name in monkeys.clone().keys() {
            if monkeys[name].num.is_some() {
                continue;
            }

            let (left, right) = {
                let monkey = monkeys.get(name).unwrap();
                (monkey.left.clone(), monkey.right.clone())
            };

            if let (Some(left_num), Some(right_num)) = (monkeys[&left].num, monkeys[&right].num) {
                let monkey = monkeys.get_mut(name).unwrap();
                match monkey.op {
                    '+' => monkey.num = Some(left_num + right_num),
                    '-' => monkey.num = Some(left_num - right_num),
                    '*' => monkey.num = Some(left_num * right_num),
                    '/' => monkey.num = Some(left_num / right_num),
                    _ => panic!("Invalid operator"),
                };
            }
        }
    }

    return monkeys[root_name].num.unwrap();
}

fn humn_yell(monkeys: &mut HashMap<String, Monkey>, root_name: &str, root_goal: i64) -> i64 {
    if root_name == "humn" {
        return root_goal;
    }

    let on_left: bool = find_humn(&monkeys, &monkeys[root_name].left);

    let other_num: i64 = if on_left {
        monkeys_yell(&mut monkeys.clone(), &monkeys[root_name].right)
    } else {
        monkeys_yell(&mut monkeys.clone(), &monkeys[root_name].left)
    };

    let goal_num: i64;
    match monkeys[root_name].op {
        '+' => goal_num = root_goal - other_num,
        '-' => {
            goal_num = if on_left {
                other_num + root_goal
            } else {
                other_num - root_goal
            }
        }
        '*' => goal_num = root_goal / other_num,
        '/' => {
            goal_num = if on_left {
                other_num * root_goal
            } else {
                other_num / root_goal
            }
        }
        _ => panic!("Invalid operator"),
    };

    return humn_yell(
        &mut monkeys.clone(),
        if on_left {
            &monkeys[root_name].left
        } else {
            &monkeys[root_name].right
        },
        goal_num,
    );
}

fn find_humn(monkeys: &HashMap<String, Monkey>, name: &String) -> bool {
    if name == "humn" {
        return true;
    }

    if !monkeys.contains_key(name) {
        return false;
    }

    let (left, right) = {
        let monkey = monkeys.get(name).unwrap();
        (monkey.left.clone(), monkey.right.clone())
    };

    return find_humn(monkeys, &left) || find_humn(monkeys, &right);
}

#[derive(Debug, Clone)]
struct Monkey {
    num: Option<i64>,
    left: String,
    right: String,
    op: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 152);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 301);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "21".to_string();
	
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
        "\nPart 1:\nRoot yells: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber to yell: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}