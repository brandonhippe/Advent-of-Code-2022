use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let direction_map: HashMap<char, (i32, i32)> =
        HashMap::from([('D', (0, 1)), ('U', (0, -1)), ('R', (1, 0)), ('L', (-1, 0))]);

    for line in contents.lines() {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[2..].parse::<i32>().unwrap();

        for _ in 0..distance {
            head = (
                head.0 + direction_map[&direction].0,
                head.1 + direction_map[&direction].1,
            );
            tail = move_tail(head, tail);

            visited.insert(tail.clone());
        }
    }

    return visited.len() as i32;
}

fn part2(contents: String) -> i32 {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let direction_map: HashMap<char, (i32, i32)> =
        HashMap::from([('D', (0, 1)), ('U', (0, -1)), ('R', (1, 0)), ('L', (-1, 0))]);

    for line in contents.lines() {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[2..].parse::<i32>().unwrap();

        for _ in 0..distance {
            knots[0] = (
                knots[0].0 + direction_map[&direction].0,
                knots[0].1 + direction_map[&direction].1,
            );
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let mut tail = knots[i];
                tail = move_tail(head, tail);
                knots[i] = tail;
            }

            visited.insert(knots.last().unwrap().clone());
        }
    }

    return visited.len() as i32;
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dist = manhat_dist(head, tail);
    if dist <= 1 || (dist == 2 && head.0 != tail.0 && head.1 != tail.1) {
        return tail;
    }

    let mut head_set = HashSet::from([
        (head.0 - 1, head.1),
        (head.0 + 1, head.1),
        (head.0, head.1 - 1),
        (head.0, head.1 + 1),
    ]);

    let tail_set = HashSet::from([
        (tail.0 - 1, tail.1),
        (tail.0 + 1, tail.1),
        (tail.0, tail.1 - 1),
        (tail.0, tail.1 + 1),
        (tail.0 - 1, tail.1 - 1),
        (tail.0 + 1, tail.1 + 1),
        (tail.0 - 1, tail.1 + 1),
        (tail.0 + 1, tail.1 - 1),
    ]);

    let mut common: HashSet<(i32, i32)> = head_set.intersection(&tail_set).cloned().collect();

    if common.len() == 0 {
        head_set = HashSet::from([
            (head.0 - 1, head.1),
            (head.0 + 1, head.1),
            (head.0, head.1 - 1),
            (head.0, head.1 + 1),
            (head.0 - 1, head.1 - 1),
            (head.0 + 1, head.1 + 1),
            (head.0 - 1, head.1 + 1),
            (head.0 + 1, head.1 - 1),
        ]);

        common = head_set.intersection(&tail_set).cloned().collect();
        return common.iter().next().unwrap().clone();
    } else {
        let mut min_dist = manhat_dist(tail, head);
        let mut new_tail: (i32, i32) = tail.clone();
        for point in common {
            let dist = manhat_dist(tail, point);
            if dist < min_dist {
                min_dist = dist;
                new_tail = point;
            }
        }

        return new_tail;
    }
}

fn manhat_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 13);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 36)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "9".to_string();
	
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
        "\nPart 1:\nSpots visited by tail: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSpots visited by tail: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}