use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut file = File {
        name: String::from("/"),
        size: 0,
        sub_dirs: HashMap::new(),
        parent: usize::MAX,
        files: Vec::new(),
    };

    let mut file_stack: Vec<File> = vec![file.clone()];
    let mut file_ix = 0;

    for line in contents.lines() {
        if line.contains(" cd ") {
            let dir = line.split(" ").nth(2).unwrap();
            if dir == ".." {
                file_stack[file_ix] = file.clone();
                file_ix = file.parent;
                file = file_stack[file_ix].clone();
            } else if dir != "/" {
                file_stack[file_ix] = file.clone();
                file_ix = file.sub_dirs.get(dir).unwrap().clone();
                file = file_stack[file_ix].clone();
            }
        } else if line != "$ ls" {
            if line.contains("dir") {
                let dir = line.split(" ").nth(1).unwrap();
                let new_dir = File {
                    name: String::from(dir),

                    size: 0,
                    sub_dirs: HashMap::new(),
                    parent: file_ix,
                    files: Vec::new(),
                };
                file.sub_dirs.insert(String::from(dir), file_stack.len());
                file_stack.push(new_dir);
            } else {
                let file_size = line.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                file.files.push(file_size);
                file.size += file_size;
            }
        }
    }

    file_stack[file_ix] = file.clone();
    calc_dir_sizes(&mut file_stack, 0);

    return file_stack
        .iter()
        .map(|f| if f.size <= 100_000 { f.size } else { 0 })
        .sum();
}

fn part2(contents: String) -> i32 {
    let mut file = File {
        name: String::from("/"),
        size: 0,
        sub_dirs: HashMap::new(),
        parent: usize::MAX,
        files: Vec::new(),
    };

    let mut file_stack: Vec<File> = vec![file.clone()];
    let mut file_ix = 0;

    for line in contents.lines() {
        if line.contains(" cd ") {
            let dir = line.split(" ").nth(2).unwrap();
            if dir == ".." {
                file_stack[file_ix] = file.clone();
                file_ix = file.parent;
                file = file_stack[file_ix].clone();
            } else if dir != "/" {
                file_stack[file_ix] = file.clone();
                file_ix = file.sub_dirs.get(dir).unwrap().clone();
                file = file_stack[file_ix].clone();
            }
        } else if line != "$ ls" {
            if line.contains("dir") {
                let dir = line.split(" ").nth(1).unwrap();
                let new_dir = File {
                    name: String::from(dir),
                    size: 0,
                    sub_dirs: HashMap::new(),
                    parent: file_ix,
                    files: Vec::new(),
                };
                file.sub_dirs.insert(String::from(dir), file_stack.len());
                file_stack.push(new_dir);
            } else {
                let file_size = line.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                file.files.push(file_size);
                file.size += file_size;
            }
        }
    }

    file_stack[file_ix] = file.clone();
    calc_dir_sizes(&mut file_stack, 0);

    return file_stack
        .iter()
        .map(|f| {
            if f.size >= 30_000_000 - (70_000_000 - file_stack[0].size) {
                f.size
            } else {
                70_000_000
            }
        })
        .min()
        .expect("Should have found a min");
}

#[derive(Clone)]
struct File {
    name: String,
    size: i32,
    sub_dirs: HashMap<String, usize>,
    parent: usize,
    files: Vec<i32>,
}

fn calc_dir_sizes(files: &mut Vec<File>, file_ix: usize) -> i32 {
    let mut file: File = files[file_ix].clone();

    for (_, sub_dir_ix) in file.sub_dirs.iter() {
        file.size += calc_dir_sizes(files, *sub_dir_ix);
    }

    files[file_ix] = file.clone();
    return file.size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 95437);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 24933642);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2022".to_string();
	let day = "7".to_string();
	
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
        "\nPart 1:\nTotal size of small directories: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSize of smallest directory to delete: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}