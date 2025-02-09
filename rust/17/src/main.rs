use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return determine_height(contents, 2022);
}

fn part2(contents: String) -> i64 {
    return determine_height(contents, 1_000_000_000_000);
}

fn determine_height(contents: String, total_rocks: i64) -> i64 {
    let jets: Vec<i64> = contents
        .chars()
        .map(|c| if c == '<' { -1 } else { 1 })
        .collect();
    let rock_templates: Vec<Vec<(i64, i64)>> = vec![
        vec![(2, 4), (3, 4), (4, 4), (5, 4)],
        vec![(2, 5), (3, 4), (3, 5), (3, 6), (4, 5)],
        vec![(2, 4), (3, 4), (4, 4), (4, 5), (4, 6)],
        vec![(2, 4), (2, 5), (2, 6), (2, 7)],
        vec![(2, 4), (2, 5), (3, 4), (3, 5)],
    ];

    let mut rock_ix: usize = 0;
    let mut jet_ix: usize = 0;

    let mut occupied: HashSet<(i64, i64)> = HashSet::new();
    let mut states: HashMap<(usize, usize), Vec<(i64, i64)>> = HashMap::new();
    let mut max_height: i64 = 0;
    let mut rock_num: i64 = 0;


    while rock_num < total_rocks {
        let mut rock = rock_templates[rock_ix]
            .clone()
            .iter()
            .map(|(x, y)| (*x, *y + max_height))
            .collect::<Vec<(i64, i64)>>();

        loop {
            let jet = jets[jet_ix];
            let jet_moved = rock
                .clone()
                .iter()
                .map(|(x, y)| (x + jet, *y))
                .collect::<Vec<(i64, i64)>>();
            if occupied
                .intersection(&jet_moved.iter().cloned().collect::<HashSet<(i64, i64)>>())
                .count()
                == 0
                && jet_moved.iter().map(|(x, _)| *x).max().unwrap() < 7
                && jet_moved.iter().map(|(x, _)| *x).min().unwrap() >= 0
            {
                rock = jet_moved;
            }

            jet_ix = (jet_ix + 1) % jets.len();

            let fallen = rock
                .clone()
                .iter()
                .map(|(x, y)| (*x, *y - 1))
                .collect::<Vec<(i64, i64)>>();
            if fallen.iter().map(|(_, y)| *y).min().unwrap() <= 0
                || fallen.iter().any(|(x, y)| occupied.contains(&(*x, *y)))
            {
                occupied = occupied
                    .union(&rock.iter().cloned().collect::<HashSet<(i64, i64)>>())
                    .cloned()
                    .collect();
                max_height = max_height.max(rock.iter().map(|(_, y)| *y).max().unwrap());
                break;
            }

            rock = fallen;
        }

        rock_num += 1;
        rock_ix = (rock_ix + 1) % rock_templates.len();

        states
            .entry((jet_ix, rock_ix))
            .or_insert(Vec::new())
            .push((rock_num as i64, max_height));
        if states[&(jet_ix, rock_ix)].len() > 2 {
            let last = states[&(jet_ix, rock_ix)].len() - 1;
            let diffs = states[&(jet_ix, rock_ix)]
                .windows(2)
                .map(|w| w[1].1 - w[0].1)
                .collect::<Vec<i64>>();

            if diffs[diffs.len() - 1] == diffs[diffs.len() - 2] {
                let cycle_len =
                    states[&(jet_ix, rock_ix)][last].0 - states[&(jet_ix, rock_ix)][last - 1].0;
                let cycles_left = (total_rocks - rock_num) / cycle_len;

                if (total_rocks - rock_num) % cycle_len == 0 {
                    return max_height + diffs[diffs.len() - 1] * cycles_left;
                }
            }
        }
    }

    return max_height;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 3068);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1514285714288);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "17".to_string();
	
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
        "\nPart 1:\nHeight after 2022 rocks: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHeight after 1,000,000,000,000 rocks: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}