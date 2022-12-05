use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const RANGE_LOWER_CASE_ASCII: (u8, u8) = (97, 122);
const RANGE_UPPER_CASE_ASCII: (u8, u8) = (65, 90);

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

    let result = match puzzle_option.as_str() {
        "A" => get_total_priority(&mut reader),
        "B" => get_total_priority_by_group(&mut reader, 3usize),
        _ => panic!("Pass a valid option! Allowed are: A | B."),
    };

    println!("Sum of priorities is: {}", result);
}

/// From a reader where each line is a string representing a rucksacks,
/// calculates priority of all content by summing priority of each rucksack.
/// Get more info on input by reading this doc: https://adventofcode.com/2022/day/3. (Part1)
fn get_total_priority(reader: &mut impl BufRead) -> u32 {
    reader.lines().fold(0u32, |mut acc, curr| {
        acc += calculate_priority_from_content(&curr.unwrap()) as u32;
        acc
    })
}

/// From a reader where each line is a string representing a rucksacks,
/// calculates priority of all content by summing priority of rucksacks in chunks of n.
/// Get more info on input by reading this doc: https://adventofcode.com/2022/day/3. (Part2)
fn get_total_priority_by_group(reader: &mut impl BufRead, chunk_size: usize) -> u32 {
    reader
        .lines()
        .chunks(chunk_size)
        .into_iter()
        .fold(0u32, |mut acc, curr| {
            acc += calculate_priority_from_contents(&curr.map(|f| f.unwrap()).collect_vec()) as u32;
            acc
        })
}

/// From string with even chars with only one item repeated first and second part of string,
/// calculates priority applying pattern described in this doc: https://adventofcode.com/2022/day/3.
fn calculate_priority_from_content(content: &str) -> u8 {
    let mut seen = [(0u8, 0u8); 53];

    for (i, c) in content.chars().enumerate() {
        if let Some(idx) = get_char_alphabetical_index(&c) {
            // we are in first part of current string.
            if i < (content.len() / 2) {
                seen[idx].0 += 1;
            } else {
                seen[idx].1 += 1;
            }
        }
    }

    seen.iter()
        .enumerate()
        .fold(0u8, |mut acc, (curr_idx, curr_value)| {
            if curr_value.0 > 0 && curr_value.1 > 0 {
                acc += curr_idx as u8;
            }
            acc
        })
}

/// From a group of strings, find the only one char that is present in all strings.
/// Calculates priority applying pattern described in this doc: https://adventofcode.com/2022/day/3 (second puzzle).
fn calculate_priority_from_contents(contents: &[String]) -> u8 {
    let mut seen = [0u8; 53];

    for (content_index, content) in contents.iter().enumerate() {
        for c in content.chars() {
            if let Some(idx) = get_char_alphabetical_index(&c) {
                // current content index should be equal to times that a char should be seen.
                if seen[idx] == content_index as u8 {
                    seen[idx] += 1;
                }
            }
        }
    }

    seen.iter()
        .enumerate()
        .fold(0u8, |mut acc, (curr_idx, curr_value)| {
            // a char to be count has to be seen in each content.
            if *curr_value == contents.len() as u8 {
                acc += curr_idx as u8;
            }
            acc
        })
}

/// From a char gets it's priority by applying pattern: a,..,z,A...,Z == 1,..26,27,..52 .
fn get_char_alphabetical_index(c: &char) -> Option<usize> {
    let current_char_ascii = *c as u8;

    if current_char_ascii >= RANGE_LOWER_CASE_ASCII.0
        && current_char_ascii <= RANGE_LOWER_CASE_ASCII.1
    {
        // normalization.
        let idx = current_char_ascii - RANGE_LOWER_CASE_ASCII.0 + 1;
        return Some(idx as usize);
    } else if current_char_ascii >= RANGE_UPPER_CASE_ASCII.0
        && current_char_ascii <= RANGE_UPPER_CASE_ASCII.1
    {
        // normalization.
        let idx = current_char_ascii - RANGE_UPPER_CASE_ASCII.0 + 27;
        return Some(idx as usize);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_priority_from_content, calculate_priority_from_contents,
        get_char_alphabetical_index, get_total_priority, get_total_priority_by_group,
    };
    use rstest::rstest;
    use std::{fs::File, io::BufReader};

    #[rstest]
    #[case('a', 1)]
    #[case('c', 3)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('D', 30)]
    #[case('Z', 52)]
    fn should_get_correct_alphabet_index_for_char(#[case] c: char, #[case] expected: usize) {
        assert_eq!(expected, get_char_alphabetical_index(&c).unwrap());
    }

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 16)]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38)]
    #[case("PmmdzqPrVvPwwTWBwg", 42)]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22)]
    #[case("ttgJtRGJQctTZtZT", 20)]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 19)]
    fn should_find_correct_priority(#[case] content: &str, #[case] expected: u8) {
        assert_eq!(expected, calculate_priority_from_content(content));
    }

    #[rstest]
    #[case([String::from("vJrwpWtwJgWrhcsFMMfFFhFp"), String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), String::from("PmmdzqPrVvPwwTWBwg")], 18)]
    #[case([String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), String::from("ttgJtRGJQctTZtZT"), String::from("CrZsJsPPZsGzwwsLwLmpwMDw")], 52)]
    fn should_find_correct_priority_for_groups(
        #[case] contents: [String; 3],
        #[case] expected: u8,
    ) {
        assert_eq!(expected, calculate_priority_from_contents(&contents))
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 3, 70)]
    fn should_calculate_correct_priority_for_file_with_group_contents(
        #[case] path: &str,
        #[case] chunk_size: usize,
        #[case] expected: u32,
    ) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(
            expected,
            get_total_priority_by_group(&mut reader, chunk_size)
        );
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 157)]
    fn should_calculate_corret_priority_for_file(#[case] path: &str, #[case] expected: u32) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_total_priority(&mut reader));
    }
}
