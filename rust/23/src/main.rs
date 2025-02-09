use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut elves: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let mut deltas = VecDeque::from(vec![
        vec![(-1, -1), (0, -1), (1, -1)],
        vec![(-1, 1), (0, 1), (1, 1)],
        vec![(-1, -1), (-1, 0), (-1, 1)],
        vec![(1, -1), (1, 0), (1, 1)],
    ]);

    for _ in 0..10 {
        elves = move_elves(elves, &mut deltas).0;
    }

    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    return (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64;

}

fn part2(contents: String) -> i64 {
    let mut elves: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let mut deltas = VecDeque::from(vec![
        vec![(-1, -1), (0, -1), (1, -1)],
        vec![(-1, 1), (0, 1), (1, 1)],
        vec![(-1, -1), (-1, 0), (-1, 1)],
        vec![(1, -1), (1, 0), (1, 1)],
    ]);

    let mut steps: i64 = 0;
    loop {
        let (new_elves, moved) = move_elves(elves, &mut deltas);

        steps += 1;
        elves = new_elves;
        if !moved {
            break;
        }
    }

    return steps;
}

fn move_elves(
    elves: HashSet<(i64, i64)>,
    deltas: &mut VecDeque<Vec<(i64, i64)>>,
) -> (HashSet<(i64, i64)>, bool) {
    let mut proposed_moved: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    let mut spot_counts: HashMap<(i64, i64), i64> = HashMap::new();
    for (x, y) in elves.iter() {
        let neighbor_count = (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .filter(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
            .count();

        if neighbor_count == 0 {
            continue;
        }

        for delta in deltas.iter() {
            let count = delta
                .iter()
                .filter(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
                .count();

            if count == 0 {
                proposed_moved.insert((*x, *y), (x + delta[1].0, y + delta[1].1));
                *spot_counts
                    .entry((x + delta[1].0, y + delta[1].1))
                    .or_insert(0) += 1;
                break;
            }
        }
    }

    deltas.rotate_left(1);

    let mut new_elves: HashSet<(i64, i64)> = HashSet::new();
    for src in elves.iter() {
        if let Some(dst) = proposed_moved.get(src) {
            if spot_counts.get(dst).unwrap() == &1 {
                new_elves.insert(*dst);
            } else {
                new_elves.insert(*src);
            }
        } else {
            new_elves.insert(*src);
        }
    }

    return (new_elves, proposed_moved.len() > 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 110);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 20);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "23".to_string();
	
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
        "\nPart 1:\nEmpty tiles in smallest rectangle: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFirst round where no elf moves: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}