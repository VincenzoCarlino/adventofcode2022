use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq)]
enum RockPaperScissorTypes {
    Rock,
    Scissor,
    Paper,
}

struct RockPaperScissorFigure {
    figure: RockPaperScissorTypes,
    who_beats_me: RockPaperScissorTypes,
}

impl RockPaperScissorFigure {
    fn new(figure_name: &char) -> Self {
        match figure_name {
            // A and X maps Rock.
            'A' | 'X' => RockPaperScissorFigure {
                figure: RockPaperScissorTypes::Rock,
                who_beats_me: RockPaperScissorTypes::Paper,
            },
            // B and Y maps Paper.
            'B' | 'Y' => RockPaperScissorFigure {
                figure: RockPaperScissorTypes::Paper,
                who_beats_me: RockPaperScissorTypes::Scissor,
            },
            // C and Z maps Scissor.
            'C' | 'Z' => RockPaperScissorFigure {
                figure: RockPaperScissorTypes::Scissor,
                who_beats_me: RockPaperScissorTypes::Rock,
            },
            _ => panic!("Only: A,B,C,X,Y,Z are allowed."),
        }
    }
}

struct RockPaperScissoGame {
    player_one_points: u8,
    player_two_points: u8,
}

impl RockPaperScissoGame {
    const DRAW_POINTS: u8 = 3;
    const VICTORY_POINTS: u8 = 6;

    fn new(player_one: &RockPaperScissorFigure, player_two: &RockPaperScissorFigure) -> Self {
        if player_one.figure == player_two.figure {
            let points =
                Self::get_value_of_rockpaperscissor_type(&player_one.figure) + Self::DRAW_POINTS;

            return RockPaperScissoGame {
                player_one_points: points,
                player_two_points: points,
            };
        }

        let player_one_figure_points = Self::get_value_of_rockpaperscissor_type(&player_one.figure);
        let player_two_figure_points = Self::get_value_of_rockpaperscissor_type(&player_two.figure);

        if player_two.who_beats_me == player_one.figure {
            return RockPaperScissoGame {
                player_one_points: player_one_figure_points + Self::VICTORY_POINTS,
                player_two_points: player_two_figure_points,
            };
        }

        RockPaperScissoGame {
            player_one_points: player_one_figure_points,
            player_two_points: player_two_figure_points + Self::VICTORY_POINTS,
        }
    }

    fn get_value_of_rockpaperscissor_type(t: &RockPaperScissorTypes) -> u8 {
        match t {
            RockPaperScissorTypes::Rock => 1,
            RockPaperScissorTypes::Paper => 2,
            RockPaperScissorTypes::Scissor => 3,
        }
    }
}

fn main() {
    // Get path file from arg passed.
    let path = std::env::args()
        .nth(1)
        .expect("You have to pass input file.");

    // Retrieve file.
    let file = File::open(&path).expect(&format!("Unable to find file: {}", &path));
    let mut reader = BufReader::new(file);

    println!(
        "Player two total points: {}",
        get_total_points_of_player_two(&mut reader)
    );
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

        sum += RockPaperScissoGame::new(&player_one, &player_two).player_two_points as u32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{get_total_points_of_player_two, RockPaperScissoGame, RockPaperScissorFigure};
    use rstest::rstest;
    use std::fs::File;
    use std::io::BufReader;

    #[rstest]
    #[case("A", "Y", 8)]
    #[case("B", "X", 1)]
    #[case("C", "Z", 6)]
    fn should_calculate_total_points_of_player_two(
        #[case] player_one: char,
        #[case] player_two: char,
        #[case] points: u8,
    ) {
        let player_one_figure = RockPaperScissorFigure::new(&player_one);
        let player_two_figure = RockPaperScissorFigure::new(&player_two);

        assert_eq!(
            points,
            RockPaperScissoGame::new(&player_one_figure, &player_two_figure).player_two_points
        );
    }

    #[rstest]
    #[case("./src/test_inputs/test_input_1.txt", 15)]
    fn should_calculate_total_points_of_player_two_from_file(
        #[case] path: &str,
        #[case] expected: u32,
    ) {
        // Retrieve file.
        let file = File::open(path).expect(&format!("Unable to find file: {}", path));
        let mut reader = BufReader::new(file);

        assert_eq!(expected, get_total_points_of_player_two(&mut reader))
    }
}
