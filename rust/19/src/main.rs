use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let indexes = [6, 12, 18, 21, 27, 30];

    let mut total: i64 = 0;
    for (i, line) in contents.lines().enumerate() {
        let costs: (i64, i64, i64, i64, i64, i64) = line
            .split(" ")
            .enumerate()
            .filter(|(i, _)| indexes.contains(i))
            .map(|(_, x)| x.parse::<i64>().unwrap())
            .collect_tuple()
            .expect("Could not parse line");
        total += (i as i64 + 1) * bfs(costs, 24);
    }

    return total;
}

fn part2(contents: String) -> i64 {
    let indexes = [6, 12, 18, 21, 27, 30];

    let mut product: i64 = 1;
    for (i, line) in contents.lines().enumerate() {
        if i == 3 {
            break;
        }

        let costs: (i64, i64, i64, i64, i64, i64) = line

            .split(" ")
            .enumerate()
            .filter(|(i, _)| indexes.contains(i))
            .map(|(_, x)| x.parse::<i64>().unwrap())
            .collect_tuple()
            .expect("Could not parse line");
        product *= most_geodes(costs, 32);
    }

    return product;
}

fn bfs(costs: (i64, i64, i64, i64, i64, i64), time: i64) -> i64 {
    let (ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = costs;

    let mut states: Vec<HashSet<(i64, i64, i64, i64, i64, i64, i64)>> =
        vec![HashSet::new(); time as usize + 1];
    states[0].insert((0, 0, 0, 0, 1, 0, 0));

    let mut max_geodes: i64 = 0;
    for t in 0..=time {
        for (geo, ore, clay, obs, ore_r, clay_r, obs_r) in states.clone()[t as usize].iter() {
            max_geodes = max_geodes.max(*geo);
            // Save up to make Geode Robot
            if *obs_r != 0 {
                let time_taken: i64 = ((geo_ore - ore) as f64 / *ore_r as f64)
                    .max((geo_obs - obs) as f64 / *obs_r as f64)
                    .ceil() as i64
                    + 1;
                if time_taken <= time - t && time_taken > 0 {
                    states[(t + time_taken) as usize].insert((
                        *geo + time - (t + time_taken),
                        ore + (ore_r * time_taken) - geo_ore,
                        clay + (clay_r * time_taken),
                        obs + (obs_r * time_taken) - geo_obs,
                        *ore_r,
                        *clay_r,
                        *obs_r,
                    ));
                }
            }

            // Save up to make Obsidian Robot
            if *clay_r != 0 && obs_r < &geo_obs {
                let time_taken: i64 = ((obs_ore - ore) as f64 / *ore_r as f64)
                    .max((obs_clay - clay) as f64 / *clay_r as f64)
                    .ceil() as i64
                    + 1;
                if time_taken <= time - t && time_taken > 0 {
                    states[(t + time_taken) as usize].insert((
                        *geo,
                        ore + (ore_r * time_taken) - obs_ore,
                        clay + (clay_r * time_taken) - obs_clay,
                        obs + (obs_r * time_taken),
                        *ore_r,
                        *clay_r,
                        *obs_r + 1,
                    ));
                }
            }

            // Save up to make Clay Robot
            if clay_r < &obs_clay {
                let time_taken: i64 = ((clay_ore - ore) as f64 / *ore_r as f64).ceil() as i64 + 1;
                if time_taken <= time - t && time_taken > 0 {
                    states[(t + time_taken) as usize].insert((
                        *geo,
                        ore + (ore_r * time_taken) - clay_ore,
                        clay + (clay_r * time_taken),
                        obs + (obs_r * time_taken),
                        *ore_r,
                        *clay_r + 1,
                        *obs_r,
                    ));
                }
            }

            // Save up to make Ore Robot
            if ore_r < &clay_ore.max(obs_ore.max(geo_ore)) {
                let time_taken: i64 = ((ore_ore - ore) as f64 / *ore_r as f64).ceil() as i64 + 1;
                if time_taken <= time - t && time_taken > 0 {
                    states[(t + time_taken) as usize].insert((
                        *geo,
                        ore + (ore_r * time_taken) - ore_ore,
                        clay + (clay_r * time_taken),
                        obs + (obs_r * time_taken),
                        *ore_r + 1,
                        *clay_r,
                        *obs_r,
                    ));
                }
            }
        }
    }

    return max_geodes;
}

fn most_geodes(costs: (i64, i64, i64, i64, i64, i64), time: i64) -> i64 {
    let (ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = costs;

    let mut states: HashSet<(i64, i64, i64, i64, i64, i64, i64)> = HashSet::new();
    states.insert((0, 0, 0, 0, 1, 0, 0));

    let mut max_geodes: i64 = 0;
    for t in 0..time {
        let mut new_states: HashSet<(i64, i64, i64, i64, i64, i64, i64)> = HashSet::new();

        for (ore, clay, obs, geo, ore_r, clay_r, obs_r) in states.iter() {
            let new_ore: i64 = *ore + ore_r;
            let new_clay: i64 = *clay + clay_r;
            let new_obs: i64 = *obs + obs_r;

            max_geodes = max_geodes.max(*geo);

            if ore >= &geo_ore && obs >= &geo_obs {
                new_states.insert((
                    new_ore - geo_ore,
                    new_clay,
                    new_obs - geo_obs,
                    *geo + time - t - 1,
                    *ore_r,
                    *clay_r,
                    *obs_r,
                ));
            } else {
                new_states.insert((new_ore, new_clay, new_obs, *geo, *ore_r, *clay_r, *obs_r));

                if *ore >= ore_ore && *ore_r <= geo_ore.max(clay_ore.max(obs_ore)) {
                    new_states.insert((
                        new_ore - ore_ore,
                        new_clay,
                        new_obs,
                        *geo,
                        *ore_r + 1,
                        *clay_r,
                        *obs_r,
                    ));
                }

                if *ore >= clay_ore && clay_r <= &obs_clay {
                    new_states.insert((
                        new_ore - clay_ore,
                        new_clay,
                        new_obs,
                        *geo,
                        *ore_r,
                        *clay_r + 1,
                        *obs_r,
                    ));
                }

                if *ore >= obs_ore && *clay >= obs_clay && obs_r <= &geo_obs {
                    new_states.insert((
                        new_ore - obs_ore,
                        new_clay - obs_clay,
                        new_obs,
                        *geo,
                        *ore_r,
                        *clay_r,
                        *obs_r + 1,
                    ));
                }
            }
        }

        states = HashSet::new();
        for state in new_states.iter() {
            if state.3 + time - t - 1 >= max_geodes {
                states.insert(*state);
            }
        }
    }

    return max_geodes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 33);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "19".to_string();
	
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
        "\nPart 1:\nTotal quality levels: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProduct of most geodes of first 3 blueprints: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}