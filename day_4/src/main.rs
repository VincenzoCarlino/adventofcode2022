use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq)]
enum PointType {
    Start,
    End,
}

#[derive(Debug)]
struct Point {
    value: u8,
    p_type: PointType,
    index: usize,
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

    let result = match puzzle_option.as_str() {
        "A" => get_count_of_pair_that_overlaps(&mut reader),
        "B" => get_count_of_pair_that_intersects(&mut reader),
        _ => panic!("Pass a valid option! Allowed are: A | B."),
    };

    println!("{}", result);
}

/// Counts number of pairs that overlaps.
/// Each line represent two pairs.
/// Each line has to be in the following example format: 12-23,32-23.
/// (2-8, 4-6) is an overlap.
/// (2-8, 5-9) is no an overlap
fn get_count_of_pair_that_overlaps(reader: &mut impl BufRead) -> u32 {
    reader.lines().fold(0u32, |mut acc, l| {
        // Convert line into array of [(u8, u8)] applying pattern matching described in fn comment.
        let pairs = l.as_ref().unwrap().split(",").enumerate().fold(
            Vec::<(u8, u8)>::new(),
            |mut acc, (_, curr)| {
                let pair = curr.split_once("-").unwrap();
                acc.push((pair.0.parse::<u8>().unwrap(), pair.1.parse::<u8>().unwrap()));

                acc
            },
        );

        let there_are_overlaps = check_if_there_is_at_least_one_overlap(&pairs);

        if there_are_overlaps {
            acc += 1;
        }

        acc
    })
}

/// Counts number of pairs that intersects.
/// Each line represent two pairs.
/// Each line has to be in the following example format: 12-23,32-23.
/// (4-6, 5-8) is in intersection.
/// (4-6, 7-8) is not an intersection.
fn get_count_of_pair_that_intersects(reader: &mut impl BufRead) -> u32 {
    reader.lines().fold(0u32, |mut acc, l| {
        // Convert line into array of [(u8, u8)] applying pattern matching described in fn comment.
        let pairs = l.as_ref().unwrap().split(",").enumerate().fold(
            Vec::<(u8, u8)>::new(),
            |mut acc, (_, curr)| {
                let pair = curr.split_once("-").unwrap();
                acc.push((pair.0.parse::<u8>().unwrap(), pair.1.parse::<u8>().unwrap()));

                acc
            },
        );

        let there_are_overlaps = check_if_there_is_at_least_one_intersection(&pairs);

        if there_are_overlaps {
            acc += 1;
        }

        acc
    })
}

/// A ""porting of sweep line algorithm"" (at least taking inspiration from it).
/// Check if in an array of pairs, returns true if at least one pair overlap another.
fn check_if_there_is_at_least_one_overlap(ranges: &[(u8, u8)]) -> bool {
    // Prepare input.
    let points = create_array_of_points(ranges);
    let mut current_start_option: Option<Point> = None;
    let mut current_end_option: Option<Point> = None;

    for point in points {
        match &current_start_option {
            Some(current_start) => {
                // If we found two starts with the same value there is an overlap.
                if point.p_type == PointType::Start && current_start.value == point.value {
                    return true;
                }

                if point.p_type == PointType::End {
                    // if current pair ends with same index of start, may be there is not an overlap.
                    // Overlap in edge cases such as: [(15,15),(15,32)] exist and are handled by previous check.
                    if point.index == current_start.index {
                        current_start_option = None;
                        current_end_option = Some(point);
                    }
                    // If current point is second arg of pair, and it's index isn't equal to current start point index it means that there
                    // is an overlap.
                    else {
                        return true;
                    }
                }
            }
            None => {
                if point.p_type == PointType::Start {
                    current_start_option = Some(point);
                    continue;
                }

                // If we found two ends with the same value there is an overlap.
                if let Some(current_end) = &current_end_option {
                    if current_end.value == point.value {
                        return true;
                    }
                };

                current_end_option = Some(point);
            }
        }
    }

    false
}

/// A ""porting of sweep line algorithm"" (at least taking inspiration from it).
/// Check if in an array of pairs, returns true if at least one pair intersect with another.
fn check_if_there_is_at_least_one_intersection(ranges: &[(u8, u8)]) -> bool {
    // Prepare input.
    let points = create_array_of_points(ranges);
    let mut current_start_option: Option<Point> = None;
    let mut current_end_option: Option<Point> = None;

    for point in points {
        match &current_start_option {
            Some(current_start) => {
                // If we found two starts with the same value there is an intersection.
                if point.p_type == PointType::Start && current_start.value == point.value {
                    return true;
                }

                if point.p_type == PointType::End {
                    // if current pair ends with same index of start, may be there is not an intersection.
                    // Intersections in edge cases such as: [(15,15),(15,32)] exist and are handled by previous check.
                    if point.index == current_start.index {
                        current_start_option = None;
                        current_end_option = Some(point);
                        continue;
                    }
                }

                // If it's a start or end different from current index there is an intersection.
                return true;
            }
            None => {
                if point.p_type == PointType::Start {
                    current_start_option = Some(point);
                    continue;
                }

                // If we found two ends with the same value there is an intersection.
                if let Some(current_end) = &current_end_option {
                    if current_end.value == point.value {
                        return true;
                    }
                };

                current_end_option = Some(point);
            }
        }
    }

    false
}

/// From an array of [(u8, u8)] creates an array of points sorted applying sweep line sort rules.
fn create_array_of_points(ranges: &[(u8, u8)]) -> Vec<Point> {
    let mut points =
        ranges
            .iter()
            .enumerate()
            .fold(Vec::<Point>::new(), |mut acc, (curr_idx, curr_value)| {
                acc.push(Point {
                    value: curr_value.0,
                    p_type: PointType::Start,
                    index: curr_idx,
                });
                acc.push(Point {
                    value: curr_value.1,
                    p_type: PointType::End,
                    index: curr_idx,
                });

                acc
            });

    sort_input_for_compare_pairs(&mut points);

    points
}

/// Given an array of points sort it applying sweep line rules.
fn sort_input_for_compare_pairs(points: &mut [Point]) {
    points.sort_unstable_by(|a, b| {
        // points have same value, the point that is a Start comes first.
        if a.value == b.value {
            if a.p_type == PointType::Start {
                return Ordering::Less;
            }

            return Ordering::Greater;
        } else if a.value < b.value {
            return Ordering::Less;
        }

        Ordering::Greater
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        check_if_there_is_at_least_one_intersection, check_if_there_is_at_least_one_overlap,
        get_count_of_pair_that_overlaps, sort_input_for_compare_pairs, Point, PointType,
    };
    use rstest::rstest;
    use std::fs::File;
    use std::io::BufReader;

    #[rstest]
    #[case(&[(11,11), (11,15)], true)]
    #[case(&[(15u8,51u8), (15,15)], true)]
    #[case(&[(2u8,4u8), (6,8)], false)]
    #[case(&[(2u8,8u8), (3,7), (4,5)], true)]
    #[case(&[(2u8,8u8), (3,7)], true)]
    #[case(&[(2u8,8u8), (10,12)], false)]
    #[case(&[(0u8,2u8), (1,2)], true)]
    #[case(&[(0u8,2u8), (5,6)], false)]
    #[case(&[(0u8,2u8), (2,6)], false)]
    #[case(&[(6u8,6u8), (4,6)], true)]
    #[case(&[(3u8,5u8), (3,5)], true)]
    #[case(&[(3u8,5u8), (2,6)], true)]
    #[case(&[(5u8,7u8), (7,9)], false)]
    #[case(&[(3u8,5u8), (1,2), (10, 20), (30, 40), (50, 60)], false)]
    #[case(&[(3u8,5u8), (1,2), (10, 20), (30, 40), (50, 60), (3, 5)], true)]
    fn should_find_overlap(#[case] input: &[(u8, u8)], #[case] expected: bool) {
        assert_eq!(expected, check_if_there_is_at_least_one_overlap(input))
    }

    #[rstest]
    #[case(&[(2,4),(6,8),(2,3),(4,5),(5,7), (7,9), (2,8), (3,7), (6,6), (4,6), (2,6), (4,8)], 4)]
    fn should_find_count_of_overlaps(#[case] input: &[(u8, u8)], #[case] expected: u32) {
        let sum = input.chunks(2).fold(0u32, |mut acc, curr| {
            if check_if_there_is_at_least_one_intersection(curr) {
                acc += 1;
            }

            acc
        });

        assert_eq!(expected, sum)
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 2)]
    fn should_get_count_of_pair_overlaps(#[case] path: &str, #[case] expected: u32) {
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_count_of_pair_that_overlaps(&mut reader));
    }

    #[rstest]
    fn should_sort() {
        let mut arr = [
            Point {
                index: 1,
                value: 5,
                p_type: PointType::Start,
            },
            Point {
                index: 1,
                value: 8,
                p_type: PointType::End,
            },
            Point {
                index: 2,
                value: 3,
                p_type: PointType::Start,
            },
            Point {
                index: 2,
                value: 7,
                p_type: PointType::End,
            },
            Point {
                index: 3,
                value: 4,
                p_type: PointType::Start,
            },
            Point {
                index: 3,
                value: 5,
                p_type: PointType::End,
            },
        ];

        sort_input_for_compare_pairs(&mut arr);

        let expected = [
            Point {
                index: 2,
                value: 3,
                p_type: PointType::Start,
            },
            Point {
                index: 3,
                value: 4,
                p_type: PointType::Start,
            },
            Point {
                index: 1,
                value: 5,
                p_type: PointType::Start,
            },
            Point {
                index: 3,
                value: 5,
                p_type: PointType::End,
            },
            Point {
                index: 2,
                value: 7,
                p_type: PointType::End,
            },
            Point {
                index: 1,
                value: 8,
                p_type: PointType::End,
            },
        ];

        for (idx, curr) in arr.iter().enumerate() {
            let p = &expected[idx];

            assert_eq!(p.index, curr.index);
            assert!(p.p_type == curr.p_type);
            assert_eq!(p.value, curr.value);
        }
    }
}
