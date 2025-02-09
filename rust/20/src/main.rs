use relative_path::RelativePath;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut order: VecDeque<i64> =
        VecDeque::from_iter(contents.lines().map(|x| x.parse::<i64>().unwrap()));
    let mut indexes: VecDeque<i64> = VecDeque::from_iter(0..order.len() as i64);

    mix(&mut order, &mut indexes);
    order.rotate_left(order.iter().position(|x| *x == 0).unwrap() as usize);

    return order[1000 % order.len()] + order[2000 % order.len()] + order[3000 % order.len()];
}

fn part2(contents: String) -> i64 {
    let mut order: VecDeque<i64> = VecDeque::from_iter(
        contents
            .lines()
            .map(|x| x.parse::<i64>().unwrap() * 811589153),
    );
    let mut indexes: VecDeque<i64> = VecDeque::from_iter(0..order.len() as i64);

    for _ in 0..10 {
        mix(&mut order, &mut indexes);
    }

    order.rotate_left(order.iter().position(|x| *x == 0).unwrap() as usize);

    return order[1000 % order.len()] + order[2000 % order.len()] + order[3000 % order.len()];
}

fn mix(order: &mut VecDeque<i64>, indexes: &mut VecDeque<i64>) {

    for i in 0..order.len() {
        let loc = indexes.iter().position(|x| *x as usize == i).unwrap();

        order.rotate_left(loc as usize);
        indexes.rotate_left(loc as usize);

        let n = order.pop_front().unwrap();
        let ix = indexes.pop_front().unwrap();

        if n > 0 {
            order.rotate_left(n as usize % order.len());
            indexes.rotate_left(n as usize % order.len());
        } else {
            order.rotate_right(n.abs() as usize % order.len());
            indexes.rotate_right(n.abs() as usize % order.len());
        }

        order.push_front(n);
        indexes.push_front(ix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1623178306);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "20".to_string();
	
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
        "\nPart 1:\nSum of indexes 1000, 2000, and 3000: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of indexes 1000, 2000, and 3000: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}