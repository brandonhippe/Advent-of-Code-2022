use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        for (x, l) in line.chars().enumerate() {
            if l == '.' {
                area.insert((x as i64 + 1, y as i64 + 1));
            } else if l == '#' {
                walls.insert((x as i64 + 1, y as i64 + 1));
            }
        }
    }

    let mut neighbors: HashMap<(i64, i64), HashMap<(i64, i64), ((i64, i64), (i64, i64))>> =
        HashMap::new();
    for pos in area.iter() {
        let mut n: HashMap<(i64, i64), ((i64, i64), (i64, i64))> = HashMap::new();
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let mut new_pos = (pos.0 + dx, pos.1 + dy);
            if area.contains(&new_pos) {
                n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
                continue;
            }


            if walls.contains(&new_pos) {
                continue;
            }

            match (dx, dy) {
                (0, 1) => {
                    new_pos = (
                        pos.0,
                        std::cmp::min(
                            area.iter()
                                .filter(|(x, _)| *x == pos.0)
                                .map(|(_, y)| *y)
                                .min()
                                .unwrap(),
                            walls
                                .iter()
                                .filter(|(x, _)| *x == pos.0)
                                .map(|(_, y)| *y)
                                .min()
                                .unwrap_or(i64::MAX),
                        ),
                    );
                    if area.contains(&new_pos) {
                        n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
                    }
                }
                (1, 0) => {
                    new_pos = (
                        std::cmp::min(
                            area.iter()
                                .filter(|(_, y)| *y == pos.1)
                                .map(|(x, _)| *x)
                                .min()
                                .unwrap(),
                            walls
                                .iter()
                                .filter(|(_, y)| *y == pos.1)
                                .map(|(x, _)| *x)
                                .min()
                                .unwrap_or(i64::MAX),
                        ),
                        pos.1,
                    );
                    if area.contains(&new_pos) {
                        n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
                    }
                }
                (0, -1) => {
                    new_pos = (
                        pos.0,
                        std::cmp::max(
                            area.iter()
                                .filter(|(x, _)| *x == pos.0)
                                .map(|(_, y)| *y)
                                .max()
                                .unwrap(),
                            walls
                                .iter()
                                .filter(|(x, _)| *x == pos.0)
                                .map(|(_, y)| *y)
                                .max()
                                .unwrap_or(i64::MIN),
                        ),
                    );
                    if area.contains(&new_pos) {
                        n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
                    }
                }
                (-1, 0) => {
                    new_pos = (
                        std::cmp::max(
                            area.iter()
                                .filter(|(_, y)| *y == pos.1)
                                .map(|(x, _)| *x)
                                .max()
                                .unwrap(),
                            walls
                                .iter()
                                .filter(|(_, y)| *y == pos.1)
                                .map(|(x, _)| *x)
                                .max()
                                .unwrap_or(i64::MIN),
                        ),
                        pos.1,
                    );
                    if area.contains(&new_pos) {
                        n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
                    }
                }
                _ => (),
            }
        }

        neighbors.insert(*pos, n);
    }

    let (pos, current_dir) = follow_path(contents.lines().last().unwrap(), neighbors);

    let dir_score = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    return 1000 * pos.1
        + 4 * pos.0
        + dir_score.iter().position(|&x| x == current_dir).unwrap() as i64;
}

fn part2(contents: String) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        for (x, l) in line.chars().enumerate() {
            if l == '.' {
                area.insert((x as i64 + 1, y as i64 + 1));
            } else if l == '#' {
                walls.insert((x as i64 + 1, y as i64 + 1));
            }
        }
    }

    let y_max = area.iter().map(|(_, y)| *y).max().unwrap();
    let x_max = area.iter().map(|(x, _)| *x).max().unwrap();

    let mut neighbors: HashMap<(i64, i64), HashMap<(i64, i64), ((i64, i64), (i64, i64))>> =
        HashMap::new();
    let side_len = ((area.len() as f64 + walls.len() as f64) / 6.0).sqrt() as i64;
    let mut internal_corners: HashSet<(i64, i64)> = HashSet::new();
    let mut corners_inside: HashSet<(i64, i64)> = HashSet::new();

    for y in
        (0..y_max).filter(|n| *n as i64 % side_len == 0 || *n as i64 % side_len == side_len - 1)
    {
        for x in
            (0..x_max).filter(|n| *n as i64 % side_len == 0 || *n as i64 % side_len == side_len - 1)
        {
            if !area.contains(&(x as i64 + 1, y as i64 + 1))
                && !walls.contains(&(x as i64 + 1, y as i64 + 1))
            {
                let neighbor_count = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .filter(|(dx, dy)| {
                        area.contains(&(x as i64 + 1 + dx, y as i64 + 1 + dy))
                            || walls.contains(&(x as i64 + 1 + dx, y as i64 + 1 + dy))
                    })
                    .collect::<Vec<_>>();

                if neighbor_count.len() == 2 {
                    internal_corners.insert((x as i64 + 1, y as i64 + 1));
                    corners_inside.insert((
                        x as i64 + 1 + neighbor_count[0].0 + neighbor_count[1].0,
                        y as i64 + 1 + neighbor_count[0].1 + neighbor_count[1].1,
                    ));
                }
            }
        }
    }

    // General solution to map the edges of a net of a cube
    let mut mapped_edges: i64 = 0;
    for ic in internal_corners.iter() {
        let ic_dirs: Vec<(i64, i64)> = [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter(|(dx, dy)| {
                area.contains(&(ic.0 + dx, ic.1 + dy)) || walls.contains(&(ic.0 + dx, ic.1 + dy))
            })
            .map(|p| *p)
            .collect();
        let mut edge_data: Vec<((i64, i64), (i64, i64), (i64, i64), Option<(i64, i64)>)> =
            Vec::new();

        // (position, off edge, move, previous face)
        edge_data.push((
            (ic.0 + ic_dirs[0].0, ic.1 + ic_dirs[0].1),
            (-ic_dirs[0].0, -ic_dirs[0].1),
            (-ic_dirs[1].0, -ic_dirs[1].1),
            None,
        ));
        edge_data.push((
            (ic.0 + ic_dirs[1].0, ic.1 + ic_dirs[1].1),
            (-ic_dirs[1].0, -ic_dirs[1].1),
            (-ic_dirs[0].0, -ic_dirs[0].1),
            None,
        ));

        while mapped_edges <= 12 {
            // Move to next unmapped edge(s)
            for ix in 0..edge_data.len() {
                loop {
                    // Rotate if reached a corner
                    if !area.contains(&edge_data[ix].0) && !walls.contains(&edge_data[ix].0) {
                        edge_data[ix].0 .0 -= edge_data[ix].2 .0;
                        edge_data[ix].0 .1 -= edge_data[ix].2 .1;

                        let rot = [((0, -1), (1, 0)), ((0, 1), (-1, 0))]
                            .iter()
                            .filter(|rot| apply_rot(**rot, edge_data[ix].1) == edge_data[ix].2)
                            .map(|rot| *rot)
                            .next()
                            .unwrap();
                        edge_data[ix].1 = edge_data[ix].2;
                        edge_data[ix].2 = apply_rot(rot, edge_data[ix].2);
                    }

                    // Move to next edge if edge is already mapped
                    let mut test_pos = edge_data[ix].0.clone();
                    while walls.contains(&test_pos) {
                        test_pos.0 += edge_data[ix].2 .0;
                        test_pos.1 += edge_data[ix].2 .1;
                    }

                    if neighbors.contains_key(&test_pos)
                        && neighbors
                            .get(&test_pos)
                            .unwrap()
                            .contains_key(&edge_data[ix].1)
                    {
                        edge_data[ix].0 = (
                            edge_data[ix].0 .0 + side_len * edge_data[ix].2 .0,
                            edge_data[ix].0 .1 + side_len * edge_data[ix].2 .1,
                        );
                    } else {
                        break;
                    }
                }
            }

            if edge_data
                .iter()
                .any(|(pos, _, _, _)| corners_inside.contains(pos))
            {
                break;
            }

            // Check if both edges are on the same face as previously mapped
            let mut both_same: bool = true;
            for ix in 0..edge_data.len() {
                let (pos, _, _, p_face) = edge_data[ix];
                let curr_face = ((pos.0 - 1) / side_len, (pos.1 - 1) / side_len);
                if p_face.is_none() || p_face.unwrap() != curr_face {
                    both_same = false;
                }

                edge_data[ix].3 = Some(curr_face);
            }

            if both_same {
                break;
            }

            // Map the edge(s)
            for _ in 0..side_len {
                if area.contains(&edge_data[0].0) && area.contains(&edge_data[1].0) {
                    neighbors
                        .entry(edge_data[0].0)
                        .or_insert(HashMap::new())
                        .insert(
                            edge_data[0].1,
                            (edge_data[1].0, (-edge_data[1].1 .0, -edge_data[1].1 .1)),
                        );
                    neighbors
                        .entry(edge_data[1].0)
                        .or_insert(HashMap::new())
                        .insert(
                            edge_data[1].1,
                            (edge_data[0].0, (-edge_data[0].1 .0, -edge_data[0].1 .1)),
                        );
                }

                edge_data[0].0 .0 += edge_data[0].2 .0;
                edge_data[0].0 .1 += edge_data[0].2 .1;
                edge_data[1].0 .0 += edge_data[1].2 .0;
                edge_data[1].0 .1 += edge_data[1].2 .1;
            }

            mapped_edges += 2;
        }
    }

    for pos in area.iter() {
        let mut n: HashMap<(i64, i64), ((i64, i64), (i64, i64))> =
            neighbors.get(pos).unwrap_or(&HashMap::new()).clone();
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if area.contains(&new_pos) {
                n.insert((*dx, *dy), (new_pos, (*dx, *dy)));
            }
        }

        neighbors.insert(*pos, n);
    }

    let (pos, current_dir) = follow_path(contents.lines().last().unwrap(), neighbors);

    let dir_score = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    return 1000 * pos.1
        + 4 * pos.0
        + dir_score.iter().position(|&x| x == current_dir).unwrap() as i64;
}

fn apply_rot(rot: ((i64, i64), (i64, i64)), dir: (i64, i64)) -> (i64, i64) {
    return (
        dir.0 * rot.0 .0 + dir.1 * rot.0 .1,
        dir.0 * rot.1 .0 + dir.1 * rot.1 .1,
    );
}

fn follow_path(
    path: &str,
    neighbors: HashMap<(i64, i64), HashMap<(i64, i64), ((i64, i64), (i64, i64))>>,
) -> ((i64, i64), (i64, i64)) {
    let mut pos = (
        neighbors
            .keys()
            .filter(|(_, y)| *y == 1)
            .map(|(x, _)| *x)
            .min()
            .unwrap(),
        1,
    );
    let mut current_dir = (1, 0);
    let mut steps = 0;

    let directions: HashMap<char, ((i64, i64), (i64, i64))> =
        HashMap::from([('R', ((0, -1), (1, 0))), ('L', ((0, 1), (-1, 0)))]);

    for c in path.chars() {
        if c.is_numeric() {
            steps = steps * 10 + c.to_digit(10).unwrap() as i64;
            continue;
        }

        for _ in 0..steps {
            if neighbors.get(&pos).unwrap().contains_key(&current_dir) {
                let (new_pos, new_dir) = *neighbors.get(&pos).unwrap().get(&current_dir).unwrap();
                pos = new_pos;
                current_dir = new_dir;
            } else {
                break;
            }
        }

        current_dir = apply_rot(*directions.get(&c).unwrap(), current_dir);
        steps = 0;
    }

    if steps != 0 {
        for _ in 0..steps {
            if neighbors.get(&pos).unwrap().contains_key(&current_dir) {
                let (new_pos, new_dir) = *neighbors.get(&pos).unwrap().get(&current_dir).unwrap();
                pos = new_pos;
                current_dir = new_dir;
            } else {
                break;
            }
        }
    }

    return (pos, current_dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 6032);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 5031);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "22".to_string();
	
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
        "\nPart 1:\nPassword (flat): {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPassword (cube): {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}