#![warn(clippy::pedantic)]

use std::error::Error;
use std::io::Write;
use std::process::exit;

use clap::{crate_authors, crate_version, App, Arg};

use sigils_of_elohim_solver::{solve_one, Position};

struct Puzzle {
    section: &'static str,
    color: &'static str,
    number: u32,
    row_count: u32,
    column_count: u32,
    tetrominoes: &'static str,
    solution: &'static str,
}

impl Puzzle {
    const fn new(
        section: &'static str,
        color: &'static str,
        number: u32,
        row_count: u32,
        column_count: u32,
        tetrominoes: &'static str,
        solution: &'static str,
    ) -> Self {
        Self {
            section,
            color,
            number,
            row_count,
            column_count,
            tetrominoes,
            solution,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Sigils of Elohim Solver - Benchmark")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Benchmark tool for the Sigils of Elohim Solver utility.")
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("No output printed to stdout"),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Print the solution with box drawing characters")
                .takes_value(false),
        )
        .get_matches();

    let quiet = matches.is_present("quiet");
    let pretty = matches.is_present("pretty");

    for puzzle in puzzles().iter() {
        let solution = solve_one(
            puzzle.row_count,
            puzzle.column_count,
            puzzle.tetrominoes.parse()?,
        )?
        .unwrap();

        let solution_string = solution.to_string();
        let is_correct = solution_string == puzzle.solution;
        if !quiet {
            print_outcome(&mut std::io::stdout(), puzzle, &solution, pretty)?;
        }

        if !is_correct {
            if quiet {
                print_outcome(&mut std::io::stderr(), puzzle, &solution, pretty)?;
            }
            eprintln!("{:?}", solution_string);
            eprintln!();
            eprintln!("Solution is incorrect.");
            eprintln!("Expected solution:");
            eprintln!("{}", puzzle.solution);
            exit(1);
        }
    }

    Ok(())
}

fn print_outcome<T: Write>(
    write: &mut T,
    puzzle: &Puzzle,
    solution: &Position,
    pretty: bool,
) -> Result<(), std::io::Error> {
    writeln!(
        write,
        "{} {} {}",
        puzzle.section, puzzle.color, puzzle.number
    )?;
    if pretty {
        writeln!(write, "{:#}", solution)
    } else {
        writeln!(write, "{}", solution)
    }
}

#[rustfmt::skip]
const fn puzzles() -> [Puzzle; 96] {
    [
        Puzzle::new("A", "cyan", 1, 4, 4, "LLZZ", "AAAB\nACBB\nCCBD\nCDDD\n"),
        Puzzle::new("A", "cyan", 2, 4, 4, "IJLZ", "ABBC\nABCC\nABCD\nADDD\n"),
        Puzzle::new("A", "cyan", 3, 5, 4, "ITTLZ", "AAAA\nBBBC\nDBCC\nDEEC\nDDEE\n"),
        Puzzle::new("A", "cyan", 4, 5, 4, "JLSZZ", "AABB\nABBC\nADCC\nDDCE\nDEEE\n"),
        Puzzle::new("A", "cyan", 5, 4, 4, "ITTL", "ABBB\nACBD\nACDD\nACCD\n"),
        Puzzle::new("A", "cyan", 6, 4, 5, "TTLSZ", "AAABB\nCABBD\nCEEDD\nCCEED\n"),
        Puzzle::new("A", "cyan", 7, 5, 4, "TTJSZ", "AAAB\nCABB\nCCBD\nCEED\nEEDD\n"),
        Puzzle::new("A", "cyan", 8, 6, 4, "IOTTJZ", "ABBB\nACCB\nACCD\nAEDD\nEEFD\nEFFF\n"),

        Puzzle::new("A", "green", 1, 6, 4, "IOTTLZ", "ABCC\nABCC\nABBD\nAEDD\nEEFD\nEFFF\n"),
        Puzzle::new("A", "green", 2, 4, 7, "ITTJJLZ", "ABBBCDD\nAEBCCDF\nAEGGCDF\nAEEGGFF\n"),
        Puzzle::new("A", "green", 3, 6, 4, "IOSZJL", "AABB\nABBC\nADDC\nEDDC\nEFFC\nEEFF\n"),
        Puzzle::new("A", "green", 4, 6, 6, "TIOTTOLTJ", "ABBCCC\nABBDEC\nADDDEE\nAFFFEG\nHHFIGG\nHHIIIG\n"),
        Puzzle::new("A", "green", 5, 6, 6, "OTTTTLLLL", "AABBBC\nAADBCC\nEEDDFC\nGEDFFF\nGEHHHI\nGGHIII\n"),
        Puzzle::new("A", "green", 6, 8, 5, "IIIIJJLLSZ", "ABCDD\nABCDE\nABCDE\nABCEE\nFFFFG\nHHHGG\nHIIGJ\nIIJJJ\n"),
        Puzzle::new("A", "green", 7, 8, 5, "IITTTTJLSZ", "ABCCC\nABDCE\nABDDE\nABDEE\nFFFGG\nHFGGI\nHJJII\nHHJJI\n"),
        Puzzle::new("A", "green", 8, 8, 6, "OOTTTTSSZZJL", "AABBCC\nAABBDC\nEEFDDC\nEFFDGG\nEFHGGI\nJHHHII\nJJKKLI\nJKKLLL\n"),

        Puzzle::new("A", "yellow", 1, 6, 6, "IOOJLSSZZ", "ABBBCC\nADDBCC\nADDEEF\nAGEEFF\nGGHHFI\nGHHIII\n"),
        Puzzle::new("A", "yellow", 2, 8, 6, "TTILLJJJOOZZ", "ABBCCD\nABBCCD\nAEEEDD\nAFEGGG\nFFHHHG\nIFHJJJ\nIKKLLJ\nIIKKLL\n"),
        Puzzle::new("A", "yellow", 3, 4, 7, "LJZZTTI", "ABBBCCC\nADBEECF\nADGGEEF\nADDGGFF\n"),
        Puzzle::new("A", "yellow", 4, 5, 4, "LLJTT", "AAAB\nCABB\nCCCB\nDDDE\nDEEE\n"),
        Puzzle::new("A", "yellow", 5, 5, 4, "LZSTT", "AAAB\nACBB\nCCDB\nCEDD\nEEED\n"),
        Puzzle::new("A", "yellow", 6, 6, 6, "IOOZZLLJJ", "ABBCCD\nABBCDD\nAEECDF\nAEEGFF\nHGGGFI\nHHHIII\n"),
        Puzzle::new("A", "yellow", 7, 10, 4, "STTTTOOILL", "ABCC\nABBC\nABDC\nAEDD\nEEFD\nGEFF\nGGFH\nGHHH\nIIJJ\nIIJJ\n"),
        Puzzle::new("A", "yellow", 8, 8, 5, "ZZSTTIILLO", "ABCDD\nABCDD\nABCCE\nABFEE\nGGFFE\nHGGFI\nHJJII\nHHJJI\n"),

        Puzzle::new("A", "red", 1, 6, 6, "OOTTLLJIS", "ABBCCC\nABBDCE\nADDDEE\nAFFGHE\nIFFGHH\nIIIGGH\n"),
        Puzzle::new("A", "red", 2, 6, 6, "ZZZLLJJTT", "AAABCC\nDABBBC\nDDDEEC\nFFGGEE\nHFFGGI\nHHHIII\n"),
        Puzzle::new("A", "red", 3, 5, 4, "IOOJJ", "ABBC\nABBC\nADCC\nADEE\nDDEE\n"),
        Puzzle::new("A", "red", 4, 6, 8, "TTOOILLJJJSS", "ABBCCDEE\nABBCCDDE\nAFFGGDHE\nAFGGHHHI\nJFKKKLLI\nJJJKLLII\n"),
        Puzzle::new("A", "red", 5, 5, 4, "TTZLI", "AAAA\nBBBC\nDBCC\nDEEC\nDDEE\n"),
        Puzzle::new("A", "red", 6, 10, 4, "TTTTZZSIIJ", "ABBB\nACBD\nACDD\nACED\nFCEE\nFFEG\nFHGG\nHHGI\nHJJI\nJJII\n"),
        Puzzle::new("A", "red", 7, 4, 7, "IITTZSL", "ABCCCDD\nABECDDF\nABEGGFF\nABEEGGF\n"),
        Puzzle::new("A", "red", 8, 6, 8, "ITTOOSZZZJJJ", "ABBCCDDD\nABBCCEDF\nAGGHEEFF\nAGHHIEFJ\nKGHIILLJ\nKKKILLJJ\n"),

        Puzzle::new("B", "cyan", 1, 4, 4, "OOLL", "AABB\nAABB\nCCCD\nCDDD\n"),
        Puzzle::new("B", "cyan", 2, 4, 5, "LLZZI", "ABBBC\nABDCC\nADDCE\nADEEE\n"),
        Puzzle::new("B", "cyan", 3, 4, 5, "JJLLI", "ABCCC\nABBBC\nADDDE\nADEEE\n"),
        Puzzle::new("B", "cyan", 4, 4, 6, "JLSTTI", "ABBBCC\nADBCCE\nADFFFE\nADDFEE\n"),
        Puzzle::new("B", "cyan", 5, 4, 4, "IOLJ", "ABBB\nACCB\nACCD\nADDD\n"),
        Puzzle::new("B", "cyan", 6, 5, 4, "TTZZL", "ABBB\nAABC\nADCC\nDDCE\nDEEE\n"),
        Puzzle::new("B", "cyan", 7, 6, 4, "JLSOII", "ABBB\nACCB\nACCD\nAEED\nEEFD\nFFFD\n"),
        Puzzle::new("B", "cyan", 8, 4, 4, "ILJZ", "ABBC\nABCC\nABCD\nADDD\n"),

        Puzzle::new("B", "green", 1, 4, 5, "LLLJZ", "AABCC\nABBDC\nABEDC\nEEEDD\n"),
        Puzzle::new("B", "green", 2, 4, 6, "TTSSZL", "AAABBC\nDABBCC\nDDEECF\nDEEFFF\n"),
        Puzzle::new("B", "green", 3, 6, 6, "IOLLLLJTT", "ABBCCC\nABBDCE\nADDDEE\nAFFFGE\nHFGGGI\nHHHIII\n"),
        Puzzle::new("B", "green", 4, 5, 8, "OOOOOOOLLI", "AAAABBCC\nDDEEBBCC\nDDEEFFGG\nHHIIFFJG\nHHIIJJJG\n"),
        Puzzle::new("B", "green", 5, 6, 6, "LLJZTTOOI", "ABBCCC\nABBDCE\nAFFDDE\nAGFDEE\nGGFHII\nGHHHII\n"),
        Puzzle::new("B", "green", 6, 5, 8, "OOLLLJIIIS", "ABCCCCDD\nABEEEFDD\nABEGHFFF\nABIGHHJJ\nIIIGGHJJ\n"),
        Puzzle::new("B", "green", 7, 6, 6, "JJJJZZOOO", "AABBCC\nAABBCC\nDDDEEE\nFFDGGE\nHFFIGG\nHHHIII\n"),
        Puzzle::new("B", "green", 8, 5, 8, "TTTTOOSZJI", "ABBCCDDE\nABBCCDEE\nAFFFGDHE\nAIFGGJHH\nIIIGJJJH\n"),

        Puzzle::new("B", "yellow", 1, 4, 10, "OJTTSSZZII", "ABBBBCCDDE\nAFFFCCDDEE\nAGGFHIIJJE\nAGGHHHIIJJ\n"),
        Puzzle::new("B", "yellow", 2, 4, 5, "JTTSZ", "AABBC\nABBCC\nADEEC\nDDDEE\n"),
        Puzzle::new("B", "yellow", 3, 7, 4, "ITTTTZO", "AAAB\nCABB\nCDDB\nCDDE\nCFEE\nFFGE\nFGGG\n"),
        Puzzle::new("B", "yellow", 4, 4, 10, "TTZSSLLLIO", "ABBBCCDEFF\nAGBCCDDEFF\nAGGHHDIEEJ\nAGHHIIIJJJ\n"),
        Puzzle::new("B", "yellow", 5, 8, 6, "LJJJJIOOTTSS", "ABBCCD\nABBCCD\nAEEEDD\nAFEGGG\nFFHHHG\nIFJJHK\nIJJLLK\nIILLKK\n"),
        Puzzle::new("B", "yellow", 6, 4, 10, "OJZZSSTTII", "ABBBBCCDDE\nAFFFCCDDEE\nAGGFHIIJJE\nAGGHHHIIJJ\n"),
        Puzzle::new("B", "yellow", 7, 7, 4, "TOTIZTT", "AAAB\nCABB\nCDDB\nCDDE\nCFEE\nFFGE\nFGGG\n"),
        Puzzle::new("B", "yellow", 8, 6, 8, "TTTTOOZJJLLI", "ABBCCDDD\nABBCCEDF\nAGGHEEFF\nAGHHHEIF\nJGKKIIIL\nJJJKKLLL\n"),

        Puzzle::new("B", "red", 1, 4, 7, "TTTTSSI", "ABBBCCD\nAEBCCDD\nAEEFFGD\nAEFFGGG\n"),
        Puzzle::new("B", "red", 2, 10, 4, "ZZLLLIITTS", "ABCC\nABDC\nABDC\nABDD\nEEEF\nGEFF\nGGFH\nIGHH\nIJJH\nIIJJ\n"),
        Puzzle::new("B", "red", 3, 10, 4, "LLLSSZZZTT", "AAAB\nCABB\nCDDB\nCCDD\nEEFF\nGEEF\nGGHF\nIGHH\nIJJH\nIIJJ\n"),
        Puzzle::new("B", "red", 4, 5, 4, "ITTLZ", "AAAA\nBBBC\nDBCC\nDEEC\nDDEE\n"),
        Puzzle::new("B", "red", 5, 5, 8, "TTTTOOSZJI", "ABBCCDDE\nABBCCDEE\nAFFFGDHE\nAIFGGJHH\nIIIGJJJH\n"),
        Puzzle::new("B", "red", 6, 5, 8, "TTTTLJSSZZ", "AAABCDDD\nEABBCCDF\nEEGBCHFF\nIEGGHHFJ\nIIIGHJJJ\n"),
        Puzzle::new("B", "red", 7, 4, 10, "OJZZSSTTII", "ABBBBCCDDE\nAFFFCCDDEE\nAGGFHIIJJE\nAGGHHHIIJJ\n"),
        Puzzle::new("B", "red", 8, 6, 8, "TTTTOOZJJLLI", "ABBCCDDD\nABBCCEDF\nAGGHEEFF\nAGHHHEIF\nJGKKIIIL\nJJJKKLLL\n"),

        Puzzle::new("C", "cyan", 1, 4, 4, "SSJJ", "AABB\nABBC\nADDC\nDDCC\n"),
        Puzzle::new("C", "cyan", 2, 4, 4, "TTLZ", "AAAB\nCABB\nCDDB\nCCDD\n"),
        Puzzle::new("C", "cyan", 3, 5, 4, "JJLLO", "AABB\nAACB\nDDCB\nDCCE\nDEEE\n"),
        Puzzle::new("C", "cyan", 4, 8, 5, "TTZSSIIOLJ", "ABCDD\nABCDD\nABCCE\nABFEE\nGFFHE\nGGFHH\nIGJJH\nIIIJJ\n"),
        Puzzle::new("C", "cyan", 5, 7, 4, "IIITTJO", "ABCC\nABCC\nABDE\nABDE\nFDDE\nFFGE\nFGGG\n"),
        Puzzle::new("C", "cyan", 6, 6, 6, "LLJJOOOOI", "ABBCCD\nABBCCD\nAEEFDD\nAEEFGG\nHHFFIG\nHHIIIG\n"),
        Puzzle::new("C", "cyan", 7, 6, 6, "LLLLLLLLI", "ABCCCD\nABCDDD\nABBEFF\nAEEEGF\nHHHIGF\nHIIIGG\n"),
        Puzzle::new("C", "cyan", 8, 8, 5, "TSSTTISZTS", "ABBBC\nADBCC\nADDEC\nAFDEE\nGFFHE\nGGFHH\nGIJJH\nIIIJJ\n"),

        Puzzle::new("C", "green", 1, 6, 4, "SSSSJJ", "ABBB\nAACB\nDACC\nDDEC\nFDEE\nFFFE\n"),
        Puzzle::new("C", "green", 2, 5, 4, "IOLSL", "AAAB\nACCB\nCCDB\nEEDB\nEEDD\n"),
        Puzzle::new("C", "green", 3, 4, 5, "ZTTSJ", "AABBC\nABBCC\nADEEC\nDDDEE\n"),
        Puzzle::new("C", "green", 4, 4, 6, "LLLJIO", "ABBBCC\nADDBEC\nADDFEC\nAFFFEE\n"),
        Puzzle::new("C", "green", 5, 6, 6, "TILITOSIO", "ABCCDD\nABCCDD\nABEFFF\nABEEFG\nHHHEGG\nHIIIIG\n"),
        Puzzle::new("C", "green", 6, 5, 8, "JILOIJLLTT", "ABCCDEEE\nABCCDDEF\nABGGDFFF\nABHGIJJJ\nHHHGIIIJ\n"),
        Puzzle::new("C", "green", 7, 6, 6, "OJJOISZJJ", "ABBCCC\nABBDDC\nAEEFDD\nAEEFFG\nHIIIFG\nHHHIGG\n"),
        Puzzle::new("C", "green", 8, 6, 6, "TSTSOSTST", "AAABBC\nDABBCC\nDDEEFC\nGDEEFF\nGGHHIF\nGHHIII\n"),

        Puzzle::new("C", "yellow", 1, 10, 4, "SJTIIZTJJO", "ABBC\nABBC\nADDC\nADEC\nFDEE\nFFGE\nFGGG\nHHIJ\nHIIJ\nHIJJ\n"),
        Puzzle::new("C", "yellow", 2, 8, 5, "OLSLLLITJT", "ABBCC\nABBDC\nAEFDC\nAEFDD\nEEFFG\nHHHGG\nHIIJG\nIIJJJ\n"),
        Puzzle::new("C", "yellow", 3, 6, 6, "JTLZSOSJT", "AABBBC\nAADBCC\nEEDDFC\nEGHDFF\nEGHIIF\nGGHHII\n"),
        Puzzle::new("C", "yellow", 4, 5, 4, "ZLIOL", "AAAB\nACCB\nDCCB\nDEEB\nDDEE\n"),
        Puzzle::new("C", "yellow", 5, 6, 4, "ZLSLJJ", "AABB\nACCB\nACDB\nECDD\nEFFD\nEEFF\n"),
        Puzzle::new("C", "yellow", 6, 6, 6, "TTJZJLTTL", "AAABBB\nCADDBE\nCCCDEE\nFFFDEG\nFHIIIG\nHHHIGG\n"),
        Puzzle::new("C", "yellow", 7, 8, 5, "TSSOITSOZJ", "ABBCC\nABBCC\nADDDE\nAFDEE\nGFFHE\nGGFHH\nIGJJH\nIIIJJ\n"),
        Puzzle::new("C", "yellow", 8, 7, 4, "LOITTZI", "ABBC\nABBC\nADDC\nAEDC\nEEDF\nEGFF\nGGGF\n"),

        Puzzle::new("C", "red", 1, 4, 5, "LTZST", "AAABB\nCABBD\nCEEDD\nCCEED\n"),
        Puzzle::new("C", "red", 2, 6, 6, "LLIIIIIII", "ABCCCC\nABDDDD\nABEEEF\nABEGGF\nHHHHGF\nIIIIGF\n"),
        Puzzle::new("C", "red", 3, 5, 4, "JJJZJ", "ABBB\nAAAB\nCCDE\nCDDE\nCDEE\n"),
        Puzzle::new("C", "red", 4, 6, 4, "TZSLTI", "ABCC\nABBC\nABDC\nAEDD\nEEFD\nEFFF\n"),
        Puzzle::new("C", "red", 5, 6, 6, "LIOLLSZJO", "ABBCCC\nABBDEC\nAFFDEE\nAGFDDE\nGGFHII\nGHHHII\n"),
        Puzzle::new("C", "red", 6, 8, 5, "ZTTLOIIJLI", "ABCDD\nABCDD\nABCEE\nABCFE\nGGFFE\nHGGFI\nHJJJI\nHHJII\n"),
        Puzzle::new("C", "red", 7, 6, 6, "OSSSSLLLL", "AAABCC\nADDBBC\nDDEEBC\nFGEEHH\nFGGHHI\nFFGIII\n"),
        Puzzle::new("C", "red", 8, 5, 8, "LJSZTTIIOO", "ABCCCDEE\nABFCDDEE\nABFFDGGH\nABIFJGGH\nIIIJJJHH\n"),
    ]
}
