use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_fs(reader: &mut impl BufRead) -> HashMap<String, u128> {
    let mut fs = HashMap::new();

    let current_path = &mut vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$") {
            if line.contains("cd") {
                let dir = line.split(" ").last().as_ref().unwrap().to_string();
                if dir == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                    fs.entry(current_path.join("/")).or_insert(0u128);
                }
            }
        } else {
            if !line.starts_with("dir") {
                let s = line.split(" ").collect::<Vec<&str>>();
                let file_size = s.first().unwrap().parse::<u128>().unwrap();
                let path = current_path.join("/");

                for (key, val) in fs.iter_mut() {
                    if path.starts_with(key) {
                        *val += file_size;
                    }
                }
            }
        }
    }

    fs
}

fn get_size_of_smallest_dir_that_leaves_unused_space_of(
    fs: &HashMap<String, u128>,
    fs_size: u128,
    unused_space: u128,
) -> Option<u128> {
    let mut sizes = fs.values().map(|f| f).collect::<Vec<&u128>>();
    let max = sizes.iter().max().unwrap();
    let space_to_release = unused_space - (fs_size - **max);

    sizes.sort();

    for i in sizes {
        if *i >= space_to_release {
            return Some(*i);
        }
    }

    None
}

fn get_sum_of_paths_of_size_at_most(fs: &HashMap<String, u128>, max_size: u128) -> u128 {
    fs.values()
        .enumerate()
        .map(|f| f.1)
        .filter(|f| **f <= max_size)
        .sum::<u128>()
}

fn main() {
    let file = File::open("./src/test_inputs/test_input_1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let fs = get_fs(&mut reader);

    println!("{}", get_sum_of_paths_of_size_at_most(&fs, 100000));
    println!(
        "{}",
        get_size_of_smallest_dir_that_leaves_unused_space_of(&fs, 70000000, 30000000).unwrap()
    );
}
