use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Get path file from arg passed.
    let path = std::env::args()
        .nth(1)
        .expect("You have to pass input file.");

    // Retrieve file.
    let file = File::open(&path).expect(&format!("Unable to find file: {}", &path));

    match get_maximum(&file) {
        Ok(max) => println!("Biggest sequence value is: {}", max),
        Err(err) => panic!("Oooops: {}", err),
    }
}

/// Returns the value of the sequence with biggest sum.
/// Accept a file where for each single line is admited only one integer or an empty line.
/// Each sequence ends by an empty line.
fn get_maximum(file: &File) -> Result<usize, String> {
    let reader = BufReader::new(file);
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

#[cfg(test)]
mod tests {
    use crate::get_maximum;
    use rstest::*;
    use std::fs::File;

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 1001)]
    #[case("./src/test_inputs/test_input_2.txt", 35)]
    #[case("./src/test_inputs/test_input_3.txt", 6)]
    fn get_maximum_success_tests(#[case] path: &str, #[case] expected: usize) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));

        assert_eq!(expected, get_maximum(&file).unwrap());
    }
}
