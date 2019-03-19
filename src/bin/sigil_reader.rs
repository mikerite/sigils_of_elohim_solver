#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
use clap::{crate_authors, crate_version, App, Arg};
use image::{self, Rgb, RgbImage};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::process::exit;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
// A rectangle with top-left point (x1, y1) and bottom-right point (x2, y2).
struct Rect {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

struct Color {
    name: &'static str,
    range: [RangeInclusive<u8>; 3],
}

const WHITE: Color = Color {
    name: "white",
    range: [170..=255, 160..=255, 145..=255],
};

const GOLD: Color = Color {
    name: "gold",
    range: [110..=155, 95..=255, 55..=100],
};

const CYAN: Color = Color {
    name: "cyan",
    range: [0..=35, 80..=255, 100..=255],
};

const GREEN: Color = Color {
    name: "green",
    range: [0..=60, 45..=255, 0..=40],
};

const YELLOW: Color = Color {
    name: "yellow",
    range: [100..=255, 50..=255, 0..=90],
};

const RED: Color = Color {
    name: "red",
    range: [70..=255, 0..=30, 0..=30],
};

const TETROMINO_COLORS: [&Color; 4] = [&CYAN, &GREEN, &YELLOW, &RED];

const SHAPES: [(&str, [bool; 6]); 6] = [
    ("I/O", [true, true, true, true, true, true]),
    ("T", [false, true, false, true, true, true]),
    ("J", [true, true, true, false, false, true]),
    ("L", [true, true, true, true, false, false]),
    ("S", [false, true, true, true, true, false]),
    ("Z", [true, true, false, false, true, true]),
];

impl Rect {
    fn width(&self) -> u32 {
        self.x2 - self.x1 + 1
    }

    fn height(&self) -> u32 {
        self.y2 - self.y1 + 1
    }

    fn pixel_count(&self) -> u32 {
        self.width() * self.height()
    }

    fn grid(&self) -> [Self; 6] {
        let col1_start = self.x1;
        let col1_end = col1_start + self.width() / 3;
        let col2_start = col1_end + 1;
        let col2_end = col2_start + self.width() / 3;
        let col3_start = col2_end + 1;
        let col3_end = self.x2;

        let row1_start = self.y1;
        let row1_end = self.y1 + self.height() / 2;
        let row2_start = row1_end + 1;
        let row2_end = self.y2;

        [
            Self {
                x1: col1_start,
                x2: col1_end,
                y1: row1_start,
                y2: row1_end,
            },
            Self {
                x1: col2_start,
                x2: col2_end,
                y1: row1_start,
                y2: row1_end,
            },
            Self {
                x1: col3_start,
                x2: col3_end,
                y1: row1_start,
                y2: row1_end,
            },
            Self {
                x1: col1_start,
                x2: col1_end,
                y1: row2_start,
                y2: row2_end,
            },
            Self {
                x1: col2_start,
                x2: col2_end,
                y1: row2_start,
                y2: row2_end,
            },
            Self {
                x1: col3_start,
                x2: col3_end,
                y1: row2_start,
                y2: row2_end,
            },
        ]
    }
}

fn main() {
    let matches = App::new("Sigils of Elohim Solver Reader")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Outputs the puzzle data from screenshots of the video game 'Sigils of Elohim'")
        .arg(
            Arg::with_name("path")
                .help("Path to the screenshot")
                .required(true),
        )
        .get_matches();

    let path = matches.value_of_os("path").unwrap();
    let img = image::open(path).unwrap();

    let img = img.to_rgb();
    let (width, height) = img.dimensions();

    let mut white_squares = HashSet::new();
    let mut tetrominoes = vec![];
    let mut progress_dot_count: u32 = 0;
    let mut colors = HashSet::new();

    // Point that have already been processed
    let mut checked_points = HashSet::new();

    // Skip the title bar
    let start_y = 40;
    for y in start_y..height - 60 {
        for x in 0..width {
            if let Some(white_square) = get_bounds(&img, &mut checked_points, x, y, &WHITE, 13, 0.1)
            {
                white_squares.insert(white_square);
            }

            for color in &TETROMINO_COLORS {
                if let Some(tetromino) = get_bounds(&img, &mut checked_points, x, y, color, 10, 0.5)
                {
                    colors.insert(color.name);

                    let grid = tetromino.grid();

                    let counts: Vec<_> = grid
                        .iter()
                        .map(|r| {
                            let on_count = count_pixels(&img, &r, color);
                            let off_count = r.pixel_count() - on_count;
                            (off_count, on_count)
                        })
                        .collect();

                    let (mut best_shape, _) = SHAPES
                        .iter()
                        .max_by_key(|(_, grid)| {
                            grid.iter()
                                .zip(&counts)
                                .map(
                                    |(&is_on, (off_count, on_count))| {
                                        if is_on {
                                            on_count
                                        } else {
                                            off_count
                                        }
                                    },
                                )
                                .sum::<u32>()
                        })
                        .unwrap();

                    if best_shape == "I/O" {
                        best_shape = if tetromino.width() > 3 * tetromino.height() {
                            "I"
                        } else {
                            "O"
                        };
                    }

                    tetrominoes.push((best_shape, tetromino));
                }
            }
        }
    }

    tetrominoes.sort_by(|(_, a), (_, b)| {
        if a.y2 < b.y1 {
            Ordering::Less
        } else if a.y1 > b.y2 {
            Ordering::Greater
        } else {
            a.x1.cmp(&b.x1)
        }
    });

    // Search for level dots at the bottom of the screen
    for y in height - 60..height {
        for x in 0..width {
            if get_bounds(&img, &mut checked_points, x, y, &GOLD, 5, 0.1).is_some() {
                progress_dot_count += 1;
            }
        }
    }

    if white_squares.is_empty() {
        println!("Unable to find board");
        exit(1);
    }

    // Estimate the width of a square
    let sample_total: u32 = white_squares.iter().map(|s| s.width()).sum::<u32>()
        + white_squares.iter().map(|s| s.height()).sum::<u32>();
    let sample_count = 2 * white_squares.len() as u32;
    let square_width: f64 = f64::from(sample_total) / f64::from(sample_count);

    let x1 = white_squares.iter().map(|s| s.x1).min().unwrap();
    let x2 = white_squares.iter().map(|s| s.x2).max().unwrap();
    let board_width = x2 - x1;
    let column_count = (f64::from(board_width) / square_width).round();

    let y1 = white_squares.iter().map(|s| s.y1).min().unwrap();
    let y2 = white_squares.iter().map(|s| s.y2).max().unwrap();
    let board_height = y2 - y1;
    let row_count = (f64::from(board_height) / square_width).round();

    if colors.len() != 1 {
        println!("Unable to determine level color");
        exit(1);
    }

    print!(
        "\"{}\", {}, {}, {}, \"",
        colors.iter().next().unwrap(),
        progress_dot_count,
        row_count,
        column_count
    );
    for (name, _) in tetrominoes {
        print!("{}", name);
    }
    println!("\"");
}

fn is_color(pixel: Rgb<u8>, color: &Color) -> bool {
    pixel
        .data
        .iter()
        .zip(&color.range)
        .all(|(c, r)| r.start() <= c && c <= r.end())
}

fn count_pixels(image: &RgbImage, rect: &Rect, color: &Color) -> u32 {
    let mut count = 0;
    for y in rect.y1..=rect.y2 {
        for x in rect.x1..=rect.x2 {
            if is_color(*image.get_pixel(x, y), &color) {
                count += 1;
            }
        }
    }
    count
}

// Get the bounds of object based on filter, bounding box size and the proportion
// of the bounding box occupied by the object.
fn get_bounds(
    image: &RgbImage,
    checked_points: &mut HashSet<(u32, u32)>,
    x: u32,
    y: u32,
    color: &Color,
    min_size: u32,
    min_proportion: f64,
) -> Option<Rect> {
    let pixel = *image.get_pixel(x, y);
    if !is_color(pixel, &color) {
        return None;
    }

    let (width, height) = image.dimensions();
    let mut frontier = vec![];

    let mut bounds = Rect {
        x1: x,
        y1: y,
        x2: x,
        y2: y,
    };

    let mut object_pixel_count = 0;
    frontier.push((x, y));
    while let Some((x, y)) = frontier.pop() {
        checked_points.insert((x, y));

        if is_color(*image.get_pixel(x, y), &color) {
            object_pixel_count += 1;

            bounds.x1 = bounds.x1.min(x);
            bounds.y1 = bounds.y1.min(y);
            bounds.x2 = bounds.x2.max(x);
            bounds.y2 = bounds.y2.max(y);

            if x > 0 {
                let p = (x - 1, y);
                if !checked_points.contains(&p) {
                    frontier.push(p);
                }
            }
            if x < width - 1 {
                let p = (x + 1, y);
                if !checked_points.contains(&p) {
                    frontier.push(p);
                }
            }
            if y > 0 {
                let p = (x, y - 1);
                if !checked_points.contains(&p) {
                    frontier.push(p);
                }
            }
            if y < height - 1 {
                let p = (x, y + 1);
                if !checked_points.contains(&p) {
                    frontier.push(p);
                }
            }
        }
    }

    if bounds.width() < min_size || bounds.height() < min_size {
        return None;
    }

    if f64::from(object_pixel_count) / f64::from(bounds.pixel_count()) < min_proportion {
        return None;
    }

    Some(bounds)
}
