use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Get path file from arg passed.
    let path = std::env::args()
        .nth(1)
        .expect("You have to pass input file.");

    let puzzle_option = std::env::args()
        .nth(2)
        .expect("You have to pass puzzle option. Availble values are: A | B.");

    // Retrieve file.
    let file = File::open(&path).expect(&format!("Unable to find file: {}", &path));
    let mut reader = BufReader::new(file);

    match puzzle_option.as_str() {
        "A" => match get_maximum(&mut reader) {
            Ok(max) => println!("Biggest sequence value is: {}", max),
            Err(err) => panic!("Oooops: {}", err),
        },
        "B" => match get_sum_of_the_n_maximum(&mut reader, 3) {
            Ok(max) => println!("The sum of highest: 3 is: {}", max),
            Err(err) => panic!("Oooops: {}", err),
        },
        _ => panic!("Pass a valid option! Allowed are: A | B."),
    }
}

/// Returns the value of the sequence with biggest sum.
/// Accept a reader where for each single line is admited only one integer or an empty line.
/// Each sequence ends by an empty line.
fn get_maximum(reader: &mut impl BufRead) -> Result<usize, String> {
    let mut max: usize = 0;
    // Accumulator for current sequence.
    let mut sum: usize = 0;

    for l in reader.lines() {
        let line = &l.map_err(|err| format!("Unable to parse line.\n\r, {}", &err))?;

        // if line is empty we have reached end of sequence. Reset accumulator and move to next iteration.
        if line.is_empty() {
            sum = 0;
            continue;
        }

        sum += line.parse::<usize>().map_err(|err| {
            format!(
                "Unable to convert line: \"{}\" in integer.\n\r{}",
                line, &err
            )
        })?;

        if sum > max {
            max = sum;
        }
    }

    Ok(max)
}

/// Returns the sum of three sequences with biggest sum.
/// Accept a reader where for each single line is admited only one integer or an empty line.
/// Each sequence ends by an empty line.
fn get_sum_of_the_n_maximum(reader: &mut impl BufRead, n: usize) -> Result<usize, String> {
    let mut highest_values = vec![0 as usize; n];
    let mut sum: usize = 0;

    for l in reader.lines() {
        let line = &l.map_err(|err| format!("Unable to parse line.\n\r, {}", &err))?;

        // if line is empty we have reached end of sequence. Reset accumulator and move to next iteration.
        if line.is_empty() {
            update_highest_values(&mut highest_values, sum);
            sum = 0;
            continue;
        }

        sum += line.parse::<usize>().map_err(|err| {
            format!(
                "Unable to convert line: \"{}\" in integer.\n\r{}",
                line, &err
            )
        })?;
    }
    // insert the last sequence sum.
    update_highest_values(&mut highest_values, sum);
    Ok(highest_values.iter().sum())
}

/// Give a sorted array (asc) add new_value if it's biggest than first element of array.
fn update_highest_values(arr: &mut [usize], new_value: usize) {
    if new_value <= arr[0] {
        return;
    }
    arr[0] = new_value;
    arr.sort();
}

#[cfg(test)]
mod tests {
    use crate::{get_maximum, get_sum_of_the_n_maximum, update_highest_values};
    use rstest::*;
    use std::fs::File;
    use std::io::BufReader;

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 1001)]
    #[case("./src/test_inputs/test_input_2.txt", 35)]
    #[case("./src/test_inputs/test_input_3.txt", 6)]
    fn get_maximum_success(#[case] path: &str, #[case] expected: usize) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_maximum(&mut reader).unwrap());
    }

    #[rstest]
    #[case(&mut [1 as usize,2,3,4], 2, &[2 as usize,2,3,4])]
    #[case(&mut [0 as usize, 0, 0], 2, &[0 as usize, 0, 2])]
    #[case(&mut [0 as usize, 0, 2], 2, &[0 as usize, 2, 2])]
    #[case(&mut [0 as usize, 2, 2], 2, &[2 as usize, 2, 2])]
    fn update_highest_values_should_add_item(
        #[case] input_array: &mut [usize],
        #[case] new_value: usize,
        #[case] output_array: &[usize],
    ) {
        update_highest_values(input_array, new_value);

        assert_eq!(&input_array, &output_array);
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_4.txt", 82)]
    #[case("./src/test_inputs/test_input_5.txt", 395)]
    #[case("./src/test_inputs/test_input_6.txt", 0)]
    fn get_sum_of_the_n_maximum_success(#[case] path: &str, #[case] expected: usize) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_sum_of_the_n_maximum(&mut reader, 3).unwrap());
    }
}
