use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq)]
enum RockPaperScissorType {
    Rock,
    Scissor,
    Paper,
}

struct RockPaperScissorFigure {
    figure: RockPaperScissorType,
    who_defeats_me: RockPaperScissorType,
    who_i_defeat: RockPaperScissorType,
}

impl RockPaperScissorFigure {
    fn new(figure_name: &char) -> Self {
        match figure_name {
            // A and X maps Rock.
            'A' | 'X' => Self::generate_from_type(&RockPaperScissorType::Rock),
            // B and Y maps Paper.
            'B' | 'Y' => Self::generate_from_type(&RockPaperScissorType::Paper),
            // C and Z maps Scissor.
            'C' | 'Z' => Self::generate_from_type(&RockPaperScissorType::Scissor),
            _ => panic!("Only: A,B,C,X,Y,Z are allowed."),
        }
    }

    fn generate_from_type(t: &RockPaperScissorType) -> Self {
        match t {
            RockPaperScissorType::Rock => RockPaperScissorFigure {
                figure: RockPaperScissorType::Rock,
                who_defeats_me: RockPaperScissorType::Paper,
                who_i_defeat: RockPaperScissorType::Scissor,
            },
            RockPaperScissorType::Scissor => RockPaperScissorFigure {
                figure: RockPaperScissorType::Scissor,
                who_defeats_me: RockPaperScissorType::Rock,
                who_i_defeat: RockPaperScissorType::Paper,
            },
            RockPaperScissorType::Paper => RockPaperScissorFigure {
                figure: RockPaperScissorType::Paper,
                who_defeats_me: RockPaperScissorType::Scissor,
                who_i_defeat: RockPaperScissorType::Rock,
            },
        }
    }
}

struct RockPaperScissorGame {
    #[allow(dead_code)]
    player_one_points: u8,
    player_two_points: u8,
}

impl RockPaperScissorGame {
    const DRAW_POINTS: u8 = 3;
    const VICTORY_POINTS: u8 = 6;

    fn new(player_one: &RockPaperScissorFigure, player_two: &RockPaperScissorFigure) -> Self {
        if player_one.figure == player_two.figure {
            let points =
                Self::get_value_of_rockpaperscissor_type(&player_one.figure) + Self::DRAW_POINTS;

            return RockPaperScissorGame {
                player_one_points: points,
                player_two_points: points,
            };
        }

        let player_one_figure_points = Self::get_value_of_rockpaperscissor_type(&player_one.figure);
        let player_two_figure_points = Self::get_value_of_rockpaperscissor_type(&player_two.figure);

        if player_two.who_defeats_me == player_one.figure {
            return RockPaperScissorGame {
                player_one_points: player_one_figure_points + Self::VICTORY_POINTS,
                player_two_points: player_two_figure_points,
            };
        }

        RockPaperScissorGame {
            player_one_points: player_one_figure_points,
            player_two_points: player_two_figure_points + Self::VICTORY_POINTS,
        }
    }

    fn generate_from_desired_verdict(
        player_one: &RockPaperScissorFigure,
        desired_verdict: &char,
    ) -> RockPaperScissorGame {
        let player_two = match desired_verdict {
            // Player two has to lose.
            'X' => RockPaperScissorFigure::generate_from_type(&player_one.who_i_defeat),
            // We need a draw.
            'Y' => RockPaperScissorFigure::generate_from_type(&player_one.figure),
            // Player two has to win.
            'Z' => RockPaperScissorFigure::generate_from_type(&player_one.who_defeats_me),
            _ => panic!("Only values: X | Y | Z are allowed."),
        };

        Self::new(player_one, &player_two)
    }

    fn get_value_of_rockpaperscissor_type(t: &RockPaperScissorType) -> u8 {
        match t {
            RockPaperScissorType::Rock => 1,
            RockPaperScissorType::Paper => 2,
            RockPaperScissorType::Scissor => 3,
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

    let result = match puzzle_option.as_str() {
        "A" => get_total_points_of_player_two(&mut reader),
        "B" => get_total_points_of_player_two_from_desired_verdict(&mut reader),
        _ => panic!("Pass a valid option! Allowed are: A | B."),
    };

    println!("Player two total points: {}", result);
}

/// Returns the sum of total points earned in each match by player two.
/// Accept a reader where for each single line represent a game of two players in rock scissor paper.
/// Allowed moves for player one: A | B | C.
/// Allowed moves for player two: X | Y | Z.
/// Example line: "A Z".
fn get_total_points_of_player_two(reader: &mut impl BufRead) -> u32 {
    let mut sum: u32 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        // Player one figure is the first char in line.
        let player_one = RockPaperScissorFigure::new(&line.chars().nth(0).unwrap());

        // Player two figure is the third char in line.
        let player_two = RockPaperScissorFigure::new(&line.chars().nth(2).unwrap());

        sum += RockPaperScissorGame::new(&player_one, &player_two).player_two_points as u32;
    }

    sum
}

fn get_total_points_of_player_two_from_desired_verdict(reader: &mut impl BufRead) -> u32 {
    let mut sum: u32 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        // Player one figure is the first char in line.
        let player_one = RockPaperScissorFigure::new(&line.chars().nth(0).unwrap());

        sum += RockPaperScissorGame::generate_from_desired_verdict(
            &player_one,
            // Verdict of the game is the third char in line.
            &line.chars().nth(2).unwrap(),
        )
        .player_two_points as u32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{
        get_total_points_of_player_two, get_total_points_of_player_two_from_desired_verdict,
        RockPaperScissorFigure, RockPaperScissorGame,
    };
    use rstest::rstest;
    use std::fs::File;
    use std::io::BufReader;

    #[rstest]
    #[case("A", "Y", 8)]
    #[case("B", "X", 1)]
    #[case("C", "Z", 6)]
    fn should_calculate_of_player_two(
        #[case] player_one: char,
        #[case] player_two: char,
        #[case] points: u8,
    ) {
        let player_one_figure = RockPaperScissorFigure::new(&player_one);
        let player_two_figure = RockPaperScissorFigure::new(&player_two);

        assert_eq!(
            points,
            RockPaperScissorGame::new(&player_one_figure, &player_two_figure).player_two_points
        );
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 15)]
    fn should_calculate_total_points_of_player_two(#[case] path: &str, #[case] expected: u32) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_total_points_of_player_two(&mut reader))
    }

    #[rstest]
    #[case('A', 'Y', 4)]
    #[case('B', 'X', 1)]
    #[case('C', 'Z', 7)]
    fn should_calculate_points_of_player_two_from_desired_output(
        #[case] player_one: char,
        #[case] desired_output: char,
        #[case] expected: u8,
    ) {
        let player_one_figure = RockPaperScissorFigure::new(&player_one);

        assert_eq!(
            expected,
            RockPaperScissorGame::generate_from_desired_verdict(
                &player_one_figure,
                &desired_output
            )
            .player_two_points
        )
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 12)]
    fn should_calculate_total_points_of_player_two_from_desired_output(
        #[case] path: &str,
        #[case] expected: u32,
    ) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(
            expected,
            get_total_points_of_player_two_from_desired_verdict(&mut reader)
        )
    }
}
