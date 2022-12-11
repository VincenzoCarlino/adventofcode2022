use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_path_sizes(reader: &mut impl BufRead) -> u128 {
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
                    let path = current_path.join("/");
                    fs.entry(path).or_insert(0u128);
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

    fs.values()
        .enumerate()
        .map(|f| f.1.clone())
        .filter(|f| f <= &100000)
        .sum::<u128>()
}

fn main() {
    let file = File::open("./src/test_inputs/test_input_1.txt").unwrap();
    let mut reader = BufReader::new(file);
    println!("{}", get_path_sizes(&mut reader));
}
