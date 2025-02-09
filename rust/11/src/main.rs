use num::Integer;
use relative_path::RelativePath;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        match i % 7 {
            0 => monkeys.push(Monkey {
                items: VecDeque::new(),
                op: ' ',
                amt: "".to_string(),
                mod_amt: 0,
                t_monkey: 0,
                f_monkey: 0,
                activity: 0,
            }),
            1 => {
                monkeys.last_mut().unwrap().items = line
                    .split(" ")
                    .skip(4)
                    .map(|x| x.replace(",", "").parse::<i64>().unwrap())
                    .collect()
            }
            2 => {
                monkeys.last_mut().unwrap().op =
                    line.split(" ").nth(6).unwrap().chars().nth(0).unwrap();
                monkeys.last_mut().unwrap().amt = line.split(" ").nth(7).unwrap().to_string();
            }
            3 => {
                monkeys.last_mut().unwrap().mod_amt =

                    line.split(" ").nth(5).unwrap().parse::<i64>().unwrap()
            }
            4 => {
                monkeys.last_mut().unwrap().t_monkey =
                    line.split(" ").nth(9).unwrap().parse::<usize>().unwrap()
            }
            5 => {
                monkeys.last_mut().unwrap().f_monkey =
                    line.split(" ").nth(9).unwrap().parse::<usize>().unwrap()
            }
            _ => (),
        }
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut mk = monkeys[i].clone();
            mk.activity += mk.items.len() as i64;

            while mk.items.len() > 0 {
                let mut item: i64 = mk.items.pop_front().unwrap();
                let amt: i64 = mk.amt.parse::<i64>().unwrap_or(item);

                match mk.op {
                    '+' => item += amt,
                    '*' => item *= amt,
                    _ => (),
                }

                item /= 3;

                if item % mk.mod_amt == 0 {
                    monkeys[mk.t_monkey].items.push_back(item);
                } else {
                    monkeys[mk.f_monkey].items.push_back(item);
                }
            }

            monkeys[i] = mk;
        }
    }

    monkeys.sort_by(|a, b| a.activity.cmp(&b.activity));

    return monkeys
        .iter()
        .skip(monkeys.len() - 2)
        .map(|x| x.activity)
        .product();
}

fn part2(contents: String) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        match i % 7 {
            0 => monkeys.push(Monkey {
                items: VecDeque::new(),
                op: ' ',
                amt: "".to_string(),
                mod_amt: 0,
                t_monkey: 0,
                f_monkey: 0,
                activity: 0,
            }),
            1 => {
                monkeys.last_mut().unwrap().items = line
                    .split(" ")
                    .skip(4)
                    .map(|x| x.replace(",", "").parse::<i64>().unwrap())
                    .collect()
            }
            2 => {
                monkeys.last_mut().unwrap().op =
                    line.split(" ").nth(6).unwrap().chars().nth(0).unwrap();
                monkeys.last_mut().unwrap().amt = line.split(" ").nth(7).unwrap().to_string();
            }
            3 => {
                monkeys.last_mut().unwrap().mod_amt =
                    line.split(" ").nth(5).unwrap().parse::<i64>().unwrap()
            }
            4 => {
                monkeys.last_mut().unwrap().t_monkey =
                    line.split(" ").nth(9).unwrap().parse::<usize>().unwrap()
            }
            5 => {
                monkeys.last_mut().unwrap().f_monkey =
                    line.split(" ").nth(9).unwrap().parse::<usize>().unwrap()
            }
            _ => (),
        }
    }

    let mut reduce_amt: i64 = 1;
    for m in monkeys.iter() {
        reduce_amt = reduce_amt.lcm(&m.mod_amt);
    }

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut mk = monkeys[i].clone();
            mk.activity += mk.items.len() as i64;

            while mk.items.len() > 0 {
                let mut item: i64 = mk.items.pop_front().unwrap();
                let amt: i64 = mk.amt.parse::<i64>().unwrap_or(item);

                match mk.op {
                    '+' => item += amt,
                    '*' => item *= amt,
                    _ => (),
                }

                item %= reduce_amt;

                if item % mk.mod_amt == 0 {
                    monkeys[mk.t_monkey].items.push_back(item);
                } else {
                    monkeys[mk.f_monkey].items.push_back(item);
                }
            }

            monkeys[i] = mk;
        }
    }

    monkeys.sort_by(|a, b| a.activity.cmp(&b.activity));
    return monkeys
        .iter()
        .skip(monkeys.len() - 2)
        .map(|x| x.activity)
        .product();
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: char,
    amt: String,
    mod_amt: i64,
    t_monkey: usize,
    f_monkey: usize,
    activity: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 10605);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2713310158);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "11".to_string();
	
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
        "\nPart 1:\nMonkey Business: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMonkey Business: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}