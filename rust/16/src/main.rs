use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use nalgebra::DMatrix;

fn part1(contents: String) -> i32 {
    let mut valves: HashMap<String, (i32, HashMap<String, i32>)> = HashMap::new();
    let mut no_flow: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in contents.lines() {
        let valve_name: String = line.chars().skip(6).take(2).collect::<String>();
        let flow_rate: i32 = line.split("=").nth(1).unwrap().split(";").nth(0).unwrap().parse().unwrap();
        let connected_valves: HashMap<String, i32> = HashMap::from_iter(line.split(" ").skip(9).collect::<String>().split(",").map(|s| (s.to_string(), 1)));
        
        if flow_rate == 0 {
            no_flow.insert(valve_name, connected_valves);
        } else {
            valves.insert(valve_name, (flow_rate, connected_valves));
        }
    }

    if no_flow.contains_key("AA") {
        valves.insert("AA".to_string(), (0, no_flow.get("AA").unwrap().clone()));
        no_flow.remove("AA");
    }

    let mut sorted_valves: Vec<String> = valves.keys().map(|s| s.to_string()).collect();
    sorted_valves.sort();

    for v1 in sorted_valves.iter() {
        for (v2, dist) in valves.clone().get(v1).unwrap().1.iter() {

            if valves.clone().contains_key(v2) {
                continue;
            }

            let connections: &mut HashMap<String, i32> = &mut valves.get_mut(v1).unwrap().1;

            let mut open_list: VecDeque<(String, i32)> = VecDeque::from_iter(no_flow.get(v2).unwrap().iter().map(|(v, dist)| (v.to_string(), *dist)));
            let mut visited: HashSet<String> = HashSet::new();

            while let Some((v3, dist2)) = open_list.pop_front() {
                if visited.contains(&v3) || v3 == *v1 {
                    continue;
                }

                visited.insert(v3.to_string());

                if !no_flow.contains_key(&v3) && dist2 + dist < *connections.get(&v3).unwrap_or(&i32::MAX) {
                    connections.insert(v3.to_string(), dist2 + dist);
                    continue;
                }

                for (v4, dist3) in no_flow.get(&v3).unwrap().iter() {
                    open_list.push_back((v4.to_string(), dist2 + dist3));
                }
            }

            connections.remove(v2);
        }
    }

    for k in sorted_valves.iter() {
        for i in sorted_valves.iter() {
            for j in sorted_valves.iter() {
                if i == j || i == k || j == k {
                    continue;
                }

                let dist1 = valves.get(i).unwrap().1.get(k).unwrap_or(&(i32::MAX >> 2)).clone();
                let dist2 = valves.get(k).unwrap().1.get(j).unwrap_or(&(i32::MAX >> 2)).clone();
                let dist3 = valves.get(i).unwrap().1.get(j).unwrap_or(&(i32::MAX >> 2)).clone();

                if dist1 + dist2 < dist3 {
                    valves.get_mut(i).unwrap().1.insert(j.to_string(), dist1 + dist2);
                }
            }
        }
    }

    for v in sorted_valves.iter().skip(1) {
        valves.get_mut(v).unwrap().1.remove("AA");
    }

    let mut memo: HashMap<(String, i32, i32), (i32, i32)> = HashMap::new();
    return most_released_dfs(&valves, "AA".to_string(), 0, 30, &mut memo).0;
}

fn part2(contents: String) -> i32 {
    let mut valves: HashMap<String, (i32, HashMap<String, i32>)> = HashMap::new();
    let mut no_flow: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in contents.lines() {
        let valve_name: String = line.chars().skip(6).take(2).collect::<String>();
        let flow_rate: i32 = line.split("=").nth(1).unwrap().split(";").nth(0).unwrap().parse().unwrap();
        let connected_valves: HashMap<String, i32> = HashMap::from_iter(line.split(" ").skip(9).collect::<String>().split(",").map(|s| (s.to_string(), 1)));
        
        if flow_rate == 0 {
            no_flow.insert(valve_name, connected_valves);
        } else {
            valves.insert(valve_name, (flow_rate, connected_valves));
        }
    }

    if no_flow.contains_key("AA") {
        valves.insert("AA".to_string(), (0, no_flow.get("AA").unwrap().clone()));
        no_flow.remove("AA");
    }

    let mut sorted_valves: Vec<String> = valves.keys().map(|s| s.to_string()).collect();
    sorted_valves.sort();

    for v1 in sorted_valves.iter() {
        for (v2, dist) in valves.clone().get(v1).unwrap().1.iter() {
            if valves.clone().contains_key(v2) {
                continue;
            }

            let connections: &mut HashMap<String, i32> = &mut valves.get_mut(v1).unwrap().1;

            let mut open_list: VecDeque<(String, i32)> = VecDeque::from_iter(no_flow.get(v2).unwrap().iter().map(|(v, dist)| (v.to_string(), *dist)));
            let mut visited: HashSet<String> = HashSet::new();

            while let Some((v3, dist2)) = open_list.pop_front() {
                if visited.contains(&v3) || v3 == *v1 {
                    continue;
                }

                visited.insert(v3.to_string());

                if !no_flow.contains_key(&v3) && dist2 + dist < *connections.get(&v3).unwrap_or(&i32::MAX) {
                    connections.insert(v3.to_string(), dist2 + dist);
                    continue;
                }

                for (v4, dist3) in no_flow.get(&v3).unwrap().iter() {
                    open_list.push_back((v4.to_string(), dist2 + dist3));
                }
            }

            connections.remove(v2);
        }
    }

    for k in sorted_valves.iter() {
        for i in sorted_valves.iter() {
            for j in sorted_valves.iter() {
                if i == j || i == k || j == k {
                    continue;
                }

                let dist1 = valves.get(i).unwrap().1.get(k).unwrap_or(&(i32::MAX >> 2)).clone();
                let dist2 = valves.get(k).unwrap().1.get(j).unwrap_or(&(i32::MAX >> 2)).clone();
                let dist3 = valves.get(i).unwrap().1.get(j).unwrap_or(&(i32::MAX >> 2)).clone();

                if dist1 + dist2 < dist3 {
                    valves.get_mut(i).unwrap().1.insert(j.to_string(), dist1 + dist2);
                }
            }
        }
    }

    for v in sorted_valves.iter().skip(1) {
        valves.get_mut(v).unwrap().1.remove("AA");
    }

    let mut memo: HashMap<(String, i32, i32), (i32, i32)> = HashMap::new();
    let (max_single, max_visited): (i32, i32) = most_released_dfs(&valves, "AA".to_string(), 0, 26, &mut memo);
    let remaining: i32 = most_released_dfs(&valves, "AA".to_string(), max_visited, 26, &mut memo).0;

    let arr_dim = sorted_valves.len();
    let mut degree: DMatrix<f64> = DMatrix::from_element(arr_dim, arr_dim, 0.0);
    let mut adj: DMatrix<f64> = DMatrix::from_element(arr_dim, arr_dim, 0.0);

    for i in 0..arr_dim {
        degree[(arr_dim + 1) * i] = valves.get(&sorted_valves[i]).unwrap().1.len() as f64;

        for (j, dist) in valves.get(&sorted_valves[i]).unwrap().1.iter() {
            let j_index = sorted_valves.iter().position(|s| s == j).unwrap();
            adj[i + arr_dim * j_index] = *dist as f64;
        }
    }

    let laplacian = degree - adj;
    let eigen_decomp = laplacian.symmetric_eigen();
    let eigenvalues = eigen_decomp.eigenvalues;
    let eigenvectors = eigen_decomp.eigenvectors;

    let mut min_ix = 0;
    let mut min_2_ix = 0;

    for ix in 0..eigenvalues.len() {
        if eigenvalues[ix] < eigenvalues[min_ix] {
            min_2_ix = min_ix;
            min_ix = ix;
        } else if eigenvalues[ix] < eigenvalues[min_2_ix] {
            min_2_ix = ix;
        }
    }

    let fiedler_vector = eigenvectors.column(min_2_ix);
    let threshold: f64 = 0.426;

    let split: i32 = sorted_valves.iter().enumerate().filter(|(i, v)| *v != "AA" && fiedler_vector[*i] > 0.0).map(|(i, _)| 1 << i).sum();
    let changing: u16 = sorted_valves.iter().enumerate().filter(|(i, v)| *v != "AA" && fiedler_vector[*i].abs() < threshold).map(|(i, _)| 1 << i).sum();
    let changing_ix: Vec<usize> = sorted_valves.iter().enumerate().filter(|(i, v)| *v != "AA" && fiedler_vector[*i].abs() < threshold).map(|(i, _)| i).collect();

    let mut max_released: i32 = max_single + remaining;

    for i in 0..(1 << changing_ix.len()) {
        let mut visited = split & !changing as i32;
        for (j, ix) in changing_ix.iter().enumerate() {
            if i & (1 << j) != 0 {
                visited ^= 1 << ix;
            }
        }

        let mut released = most_released_dfs(&valves, "AA".to_string(), visited, 26, &mut memo).0;
        if released < remaining {
            continue;
        }

        released += most_released_dfs(&valves, "AA".to_string(), !(visited) ^ 1, 26, &mut memo).0;

        if released > max_released {
            max_released = released;
        }
    }

    return max_released;
}

fn most_released_dfs(valves: &HashMap<String, (i32, HashMap<String, i32>)>, start: String, visited_init: i32, max_t: i32, memo: &mut HashMap<(String, i32, i32), (i32, i32)>) -> (i32, i32) {
    if memo.contains_key(&(start.clone(), visited_init, max_t)) {
        return memo.get(&(start, visited_init, max_t)).unwrap().clone();
    }

    let mut sorted_valves: Vec<String> = valves.keys().map(|s| s.to_string()).collect();
    sorted_valves.sort();

    let mut most_released: i32 = 0;
    let mut most_visited: i32 = 0;
    for (v, dist) in valves.get(&start).unwrap().1.iter() {
        let valve_num = 1 << sorted_valves.iter().position(|sv| sv == v).unwrap();
        if valve_num & visited_init != 0 {
            continue;
        }

        if dist + 1 >= max_t {
            continue;
        }

        let releasing = (max_t - dist - 1) * valves.get(v).unwrap().0;
        let (released, visited) = most_released_dfs(valves, v.to_string(), visited_init | valve_num, max_t - dist - 1, memo);
        if released + releasing > most_released {
            memo.insert((start.clone(), visited_init, max_t), (released + releasing, visited | valve_num));
            most_released = released + releasing;
            most_visited = visited | valve_num;
        }
    }

    return (most_released, most_visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1651);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1707);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "16".to_string();
	
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
        "\nPart 1:\nMost pressure released: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost pressure released: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}