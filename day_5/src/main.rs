use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::giant_cargo_crane::CrateMoverModel;

/// From a string get an array of all numbers.
/// Eg: "move 14 from 3 to 4" => [14,3,4].
fn get_numbers_from_string(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .filter_map(|c| c.parse::<u8>().ok())
        .collect()
}

/// Read moves from a file and apply them to Stacks.
fn make_moves(reader: &mut impl BufRead, stacks: &mut giant_cargo_crane::Stacks<char>) {
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() || line.chars().nth(0).unwrap() != 'm' {
            continue;
        }

        let move_args = get_numbers_from_string(&line);

        if move_args.len() == 3 {
            stacks.move_n(
                *move_args.get(1).unwrap(),
                *move_args.get(2).unwrap(),
                *move_args.get(0).unwrap(),
            );
        }
    }
}

fn main() {
    // Get path file from arg passed.
    let path = std::env::args()
        .nth(1)
        .expect("You have to pass input file.");

    let puzzle_option = std::env::args()
        .nth(2)
        .expect("You have to pass puzzle option. Available values are: A | B.");

    // Retrieve file.
    let file = File::open(&path).expect(&format!("Unable to find file: {}", &path));
    let mut reader = BufReader::new(file);

    let crate_mover_model = match puzzle_option.as_str() {
        "A" => CrateMoverModel::M9000,
        "B" => CrateMoverModel::M9001,
        _ => panic!("Available values for puzzle option are: A | B."),
    };
    let mut stacks = giant_cargo_crane::Stacks::<char>::generate(&mut reader, crate_mover_model);

    make_moves(&mut reader, &mut stacks);

    println!("{:?}", stacks.get_top_of_each_stack());
}

mod giant_cargo_crane {
    use regex::Regex;
    use std::{collections::HashMap, io::BufRead};

    #[derive(PartialEq, Eq)]
    pub enum CrateMoverModel {
        M9000,
        M9001,
    }

    pub struct Stacks<T> {
        stacks: HashMap<u8, Vec<T>>,
        crate_mover_model: CrateMoverModel,
    }

    impl Stacks<char> {
        /// Create stacks from a reader.
        pub fn generate(reader: &mut impl BufRead, crate_mover_model: CrateMoverModel) -> Self {
            let mut stacks_map: HashMap<u8, Vec<char>> = HashMap::new();

            for line in reader.lines().map(|l| l.unwrap()) {
                let is_first_char_empty_space = line.chars().nth(0).unwrap() == ' ';
                let is_second_char_number_one = line.chars().nth(1).unwrap() == '1';

                // We end with informations for stacks.
                if is_first_char_empty_space && is_second_char_number_one {
                    break;
                }

                for (idx, char) in get_normalized_string_from_stacks_input(&line)
                    .chars()
                    .enumerate()
                {
                    // Check that we have a valid element to push on current stack.
                    if char != '-' {
                        stacks_map
                            .entry((idx + 1usize) as u8)
                            .and_modify(|stack| {
                                stack.push(char);
                            })
                            .or_insert(vec![char]);
                    }
                }
            }

            for (_, stack) in stacks_map.iter_mut() {
                stack.reverse();
            }

            Self {
                stacks: stacks_map,
                crate_mover_model,
            }
        }

        /// Move from one stack to another the n elements on top.
        /// May be we can make this code better using Interior mutability.
        pub fn move_n(&mut self, from: u8, to: u8, n: u8) {
            let first_stack = self.stacks.get_mut(&from).unwrap();
            let mut values = Vec::<char>::new();

            for _ in 0..n {
                if let Some(v) = first_stack.pop() {
                    if self.crate_mover_model == CrateMoverModel::M9000 {
                        values.push(v);
                    }
                    else {
                        values.insert(0, v);
                    }
                }
            }

            let to_stack = self.stacks.get_mut(&to).unwrap();

            for i in values {
                to_stack.push(i);
            }
        }

        /// Reference top element of each stack into a vec.
        pub fn get_top_of_each_stack(&self) -> Vec<&char> {
            self.stacks
                .iter()
                .fold(vec![&'-'; self.stacks.len()], |mut acc, (idx, stack)| {
                    if let Some(c) = stack.last() {
                        acc[(*idx - 1) as usize] = c;
                    }
                    acc
                })
        }
    }

    /// Clear string input (check file and https://adventofcode.com/2022/day/5 puzzle input).
    /// Converts a string "   [A] [B]      [C]" in "-AB--C".
    /// Where - indicates a stack without element.
    fn get_normalized_string_from_stacks_input(input: &str) -> String {
        // Remove: [] and spaces.
        Regex::new(r"\[|\]| ")
            .unwrap()
            .replace_all(
                // Replace every 3 spaces with -.
                &Regex::new(r"   .").unwrap().replace_all(&input, "-"),
                "",
            )
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::get_normalized_string_from_stacks_input;
        use rstest::rstest;

        #[rstest]
        #[case("    [B]             [B] [S]        ", "-B---BS--")]
        #[case("    [M]             [P] [L] [B] [J]", "-M---PLBJ")]
        #[case("    [T] [R] [Z]     [H] [H] [G] [C]", "-TRZ-HHGC")]
        #[case("[B] [L] [Q] [W] [S] [L] [J] [W] [Z]", "BLQWSLJWZ")]
        fn normalize_input_string(#[case] input: &str, #[case] expected: &str) {
            assert_eq!(expected, get_normalized_string_from_stacks_input(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::{fs::File, io::BufReader};

    use crate::{make_moves, giant_cargo_crane::Stacks, giant_cargo_crane::CrateMoverModel};

    #[rstest]
    #[case("Move 2 from 1 to 5", &[2u8,1,5])]
    #[case("Move 25 from 1 to 53", &[25u8,1,53])]
    fn should_get_numbers_from_string(#[case] input: &str, #[case] expected: &[u8]) {
        assert_eq!(expected.to_vec(), crate::get_numbers_from_string(&input))
    }

    #[rstest]
    #[case("./src/test_inputs/input_test_1.txt", CrateMoverModel::M9000, &[&'C', &'M', &'Z'])]
    #[case("./src/test_inputs/input_test_1.txt", CrateMoverModel::M9001, &[&'M', &'C', &'D'])]
    fn should_make_moves(#[case] input: &str, #[case] crate_cover_model: CrateMoverModel, #[case] expected: &[&char]) {
        // Retrieve file.
        let file = File::open(input).unwrap();
        let mut reader = BufReader::new(file);
        let mut stacks = Stacks::<char>::generate(&mut reader, crate_cover_model);

        make_moves(&mut reader, &mut stacks);

        assert_eq!(expected.to_vec(), stacks.get_top_of_each_stack())
    }
}
