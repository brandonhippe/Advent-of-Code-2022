use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let start = (
        contents.lines().nth(0).unwrap().find('.').unwrap() as i64,
        0,
    );
    let end = (
        contents.lines().last().unwrap().find('.').unwrap() as i64,
        contents.lines().count() as i64 - 1,
    );
    let mut area = HashSet::new();
    let mut blizzards: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                '>' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((1, 0));
                    area.insert((x as i64, y as i64));
                }
                '<' => {
                    blizzards
                        .entry((x as i64, y as i64))

                        .or_insert(vec![])
                        .push((-1, 0));
                    area.insert((x as i64, y as i64));
                }
                '^' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((0, -1));
                    area.insert((x as i64, y as i64));
                }
                'v' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((0, 1));
                    area.insert((x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    return bfs(start, end, area, &mut blizzards);
}

fn part2(contents: String) -> i64 {
    let start = (
        contents.lines().nth(0).unwrap().find('.').unwrap() as i64,
        0,
    );
    let end = (
        contents.lines().last().unwrap().find('.').unwrap() as i64,
        contents.lines().count() as i64 - 1,
    );
    let mut area = HashSet::new();
    let mut blizzards: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                '>' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((1, 0));
                    area.insert((x as i64, y as i64));
                }
                '<' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((-1, 0));
                    area.insert((x as i64, y as i64));
                }
                '^' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((0, -1));
                    area.insert((x as i64, y as i64));
                }
                'v' => {
                    blizzards
                        .entry((x as i64, y as i64))
                        .or_insert(vec![])
                        .push((0, 1));
                    area.insert((x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    return bfs(start, end, area.clone(), &mut blizzards)
        + bfs(end, start, area.clone(), &mut blizzards)
        + bfs(start, end, area, &mut blizzards)
        + 2;
}

fn manhat_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

fn bfs(
    start: (i64, i64),
    end: (i64, i64),
    area: HashSet<(i64, i64)>,
    blizzards: &mut HashMap<(i64, i64), Vec<(i64, i64)>>,
) -> i64 {
    let min_x = area.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = area.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = *area.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *area.iter().map(|(_, y)| y).max().unwrap();

    let mut nodes = HashSet::new();
    nodes.insert(start);

    let mut dist = 0;

    while nodes.len() > 0 {
        let p_blizzards = blizzards.clone();
        blizzards.clear();
        for (pos, offsets) in p_blizzards.iter() {
            for offset in offsets {
                let mut new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if new_pos.0 == min_x {
                    new_pos.0 = max_x - 1;
                }
                if new_pos.0 == max_x {
                    new_pos.0 = min_x + 1;
                }
                if new_pos.1 == min_y {
                    new_pos.1 = max_y - 1;
                }
                if new_pos.1 == max_y {
                    new_pos.1 = min_y + 1;
                }

                blizzards.entry(new_pos).or_insert(vec![]).push(*offset);
            }
        }

        let mut new_nodes = HashSet::new();
        for node in nodes {
            if node == end {
                return dist;
            }

            let prev_count = new_nodes.len();
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)].iter() {
                let new_node = (node.0 + dx, node.1 + dy);
                if area.contains(&new_node)
                    && !new_nodes.contains(&new_node)
                    && !blizzards.contains_key(&new_node)
                {
                    new_nodes.insert(new_node);
                }
            }
        }

        dist += 1;
        nodes = new_nodes;
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 18);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 54);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "24".to_string();
	
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
        "\nPart 1:\nShortest path: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest path: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}