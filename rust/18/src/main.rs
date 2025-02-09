use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let cubes: HashSet<(i64, i64, i64)> = HashSet::from_iter(contents.lines().map(|line| {
        line.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .expect("Not 3 numbers")
    }));

    let mut surface_area: i64 = 0;
    for pos in cubes.iter() {
        for offset in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        {
            if !cubes.contains(&(pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2)) {
                surface_area += 1;
            }
        }
    }


    return surface_area;
}

fn part2(contents: String) -> i64 {
    let cubes: HashSet<(i64, i64, i64)> = HashSet::from_iter(contents.lines().map(|line| {
        line.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .expect("Not 3 numbers")
    }));

    let mut neighbor_counts: HashMap<(i64, i64, i64), i64> = HashMap::new();

    for pos in cubes.iter() {
        for offset in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        {
            if !cubes.contains(&(pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2)) {
                *neighbor_counts
                    .entry((pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
                    .or_insert(0) += 1;
            }
        }
    }

    let min_x: i64 = *neighbor_counts.keys().map(|(x, _, _)| x).min().unwrap();
    let max_x: i64 = *neighbor_counts.keys().map(|(x, _, _)| x).max().unwrap();
    let min_y: i64 = *neighbor_counts.keys().map(|(_, y, _)| y).min().unwrap();
    let max_y: i64 = *neighbor_counts.keys().map(|(_, y, _)| y).max().unwrap();
    let min_z: i64 = *neighbor_counts.keys().map(|(_, _, z)| z).min().unwrap();
    let max_z: i64 = *neighbor_counts.keys().map(|(_, _, z)| z).max().unwrap();

    let mut inside_set: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut outside_set: HashSet<(i64, i64, i64)> = HashSet::new();

    for pos in neighbor_counts.keys() {
        if inside_set.contains(pos) || outside_set.contains(pos) {
            continue;
        }

        let mut open_list: VecDeque<(i64, i64, i64)> = VecDeque::from(vec![*pos]);
        let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut inside: bool = true;

        while let Some(current) = open_list.pop_front() {
            if !inside {
                break;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            for offset in [
                (-1, 0, 0),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ]
            .iter()
            {
                let new_pos = (
                    current.0 + offset.0,
                    current.1 + offset.1,
                    current.2 + offset.2,
                );
                if !cubes.contains(&new_pos) {
                    if new_pos.0 < min_x
                        || new_pos.0 > max_x
                        || new_pos.1 < min_y
                        || new_pos.1 > max_y
                        || new_pos.2 < min_z
                        || new_pos.2 > max_z
                    {
                        inside = false;
                        break;
                    }

                    open_list.push_back(new_pos);
                }
            }
        }

        if inside {
            inside_set.extend(visited);
        } else {
            outside_set.extend(visited);
        }
    }

    return outside_set
        .iter()
        .filter(|pos| neighbor_counts.contains_key(pos))
        .map(|pos| neighbor_counts[pos])
        .sum::<i64>() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 64);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 58);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "18".to_string();
	
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
        "\nPart 1:\nTotal surface area: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nExterior surface area: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}