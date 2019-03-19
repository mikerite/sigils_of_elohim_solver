#![warn(clippy::pedantic)]

use std::fmt::Display;
use std::process;

use clap::{crate_authors, crate_version, App, Arg};

use sigils_of_elohim_solver::{solve_one, PieceCollection};

fn main() {
    let matches = App::new("Sigils of Elohim Solver")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Solves puzzles from the video game 'Sigils of Elohim'")
        .arg(
            Arg::with_name("rows")
                .help("The number of grid rows")
                .required(true),
        )
        .arg(
            Arg::with_name("columns")
                .help("The number of grid columns")
                .required(true),
        )
        .arg(
            Arg::with_name("tetrominoes")
                .help("The set tetrominoes to tile.")
                .long_help(
                    "A string consisting the names of the one-sided tetrominoes to tile.\n\
                     For example, 'IIOL' means two I tetrominoes, one O and one L tetromino.\n\
                     See https://en.wikipedia.org/wiki/Tetromino#One-sided_tetrominoes\n\
                     for images of the one-sided tetrominoes with names.",
                )
                .required(true),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Print the solution with box drawing characters")
                .takes_value(false),
        )
        .get_matches();

    let row_count = matches.value_of("rows").unwrap();
    let row_count = parse_positive_number(row_count)
        .unwrap_or_else(|_| exit_with_error("value of <rows> must be a positive integer"));

    let col_count = matches.value_of("columns").unwrap();
    let col_count = parse_positive_number(col_count)
        .unwrap_or_else(|_| exit_with_error("value of <columns> must be a positive integer"));

    let tetrominoes = matches.value_of("tetrominoes").unwrap();
    let pieces: PieceCollection = tetrominoes.parse().unwrap_or_else(|_| {
        exit_with_error(
            "value of <tetrominoes> must be consist of letters I, O, T, J, L, S or Z only",
        )
    });

    let result = solve_one(row_count, col_count, pieces);
    let solution = result.unwrap_or_else(|err| {
        exit_with_error(err);
    });
    let pretty = matches.is_present("pretty");
    let display = solution
        .map(|s| {
            if pretty {
                format!("{:#}", s)
            } else {
                format!("{}", s)
            }
        })
        .unwrap_or_else(|| "No solution".into());

    println!("{}", display);
}

fn parse_positive_number(input: &str) -> Result<u32, ()> {
    let value: u32 = input.parse().map_err(|_| ())?;
    if value == 0 {
        return Err(());
    }
    Ok(value)
}

fn exit_with_error<T: Display>(message: T) -> ! {
    eprintln!("error: {}", message);
    process::exit(1);
}
