use std::{collections::HashSet, fs::read_to_string};

/// Find first sequence of N unique chars and returns index of last char of sequence in original input.
fn find_marker(input: &str, n: usize) -> Option<usize> {
    for i in 0..input.len() - (n - 1) {
        let set: &HashSet<char> = &input[i..i + n].chars().collect();

        if set.len() == n {
            return Some(i + n);
        }
    }

    None
}

fn main() {
    // Get path file from arg passed.
    let path = std::env::args()
        .nth(1)
        .expect("You have to pass input file.");

    let marker_l = std::env::args()
        .nth(2)
        .expect("You have to pass marker length")
        .parse::<usize>()
        .expect("Marker lenght should be a number > 0.");

    // Retrieve file.
    let content = read_to_string(&path).expect(&format!("Unable to find file: {}", &path));
    match find_marker(content.as_str(), marker_l) {
        Some(idx) => println!("Find marker after: {} chars", idx),
        None => println!("Unable to find markerd"),
    }
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use rstest::rstest;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 4, 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11)]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 14, 23)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26)]
    fn should_find_marker(
        #[case] input: &str,
        #[case] marker_length: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, find_marker(input, marker_length).unwrap_or(0))
    }
}
