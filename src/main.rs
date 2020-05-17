use colored::*;
use mlog::*;

use std::collections::LinkedList;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Pattern(String, Color);

#[derive(Debug)]
struct PatternBuffer {
    patterns: LinkedList<Pattern>,
    current_color: Color,
}

enum AddOrClosest {
    Added(PatternBuffer, Pattern),
    Closest(PatternBuffer, Pattern, Levenshtein),
}

impl PatternBuffer {
    fn next_color(&mut self) -> Color {
        use Color::*;

        let color = match self.current_color {
            Red => Green,
            Green => Yellow,
            Yellow => Blue,
            Blue => Magenta,
            Magenta => Cyan,
            Cyan => White,
            White => BrightBlack,
            BrightBlack => BrightRed,
            BrightRed => BrightGreen,
            BrightGreen => BrightYellow,
            BrightYellow => BrightBlue,
            BrightBlue => BrightMagenta,
            BrightMagenta => BrightCyan,
            BrightCyan => BrightWhite,
            BrightWhite => Red,
            Black => Red,
        };

        self.current_color = color;

        color
    }

    fn add(mut self, other: &String) -> AddOrClosest {
        let color = self.next_color();
        let pattern = Pattern(other.clone(), color);
        self.patterns.push_back(pattern.clone());

        AddOrClosest::Added(self, pattern)
    }

    fn closest(&self, other: &String) -> Option<(f64, Pattern, Levenshtein)> {
        self.patterns
            .iter()
            .map(|p| (levenshtein(&p.0, &other), p))
            .map(|(l, p)| (normalize(&p.0, &other, &l), p.clone(), l))
            .max_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap())
    }

    fn add_or_closest(self, other: &String, target: f64) -> AddOrClosest {
        use AddOrClosest::*;

        match self.closest(&other) {
            None => self.add(other),
            Some((max, pattern, diff)) => {
                if max <= target {
                    self.add(other)
                } else {
                    Closest(self, pattern, diff)
                }
            }
        }
    }
}

fn colorize(diff: &Levenshtein, pattern: &Pattern) -> Vec<ColoredString> {
    use LevenshteinOp::*;

    let mut s = Vec::with_capacity(diff.len());

    for op in diff.iter() {
        match op {
            Keep(x) => s.push(format!("{}", x).color(pattern.1).dimmed()),
            Substitute(_, x) => s.push(format!("{}", x).color(pattern.1)),
            Insert(x) => s.push(format!("{}", x).color(pattern.1)),
            _ => {}
        };
    }

    s
}

enum SourceOptions {
    FromFile(String),
    FromStdin,
}

struct Args {
    source: SourceOptions,
    interactive: bool,
    similarity: f64,
}

enum IterableSource {
    FromStdin(io::Lines<BufReader<io::Stdin>>),
    FromFile(io::Lines<BufReader<File>>),
}

impl Iterator for IterableSource {
    type Item = Result<String, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterableSource::FromStdin(s) => s.next(),
            IterableSource::FromFile(f) => f.next(),
        }
    }
}

fn lines(opts: SourceOptions) -> IterableSource {
    match opts {
        SourceOptions::FromStdin => {
            return IterableSource::FromStdin(BufReader::new(io::stdin()).lines())
        }
        SourceOptions::FromFile(filename) => {
            return IterableSource::FromFile(BufReader::new(File::open(filename).unwrap()).lines())
        }
    }
}

fn run(args: Args) -> Result<PatternBuffer, io::Error> {
    use AddOrClosest::*;

    let mut buffer = PatternBuffer {
        patterns: LinkedList::new(),
        current_color: Color::BrightBlack,
    };

    for line in lines(args.source) {
        let l = line?;
        if l.len() > 0 {
            match buffer.add_or_closest(&l, args.similarity) {
                Closest(b, p, diff) => {
                    buffer = b;
                    if args.interactive {
                        for s in colorize(&diff, &p).iter() {
                            print!("{}", s)
                        }
                        println!("")
                    }
                }
                Added(b, p) => {
                    buffer = b;

                    if args.interactive {
                        println!("{}", l.color(p.1))
                    }
                }
            }
        }
    }
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_args = match args.len() {
        1 => {
            (Args {
                source: SourceOptions::FromStdin,
                interactive: true,
                similarity: 0.6,
            })
        }
        _ => {
            (Args {
                source: SourceOptions::FromFile(args[1].clone()),
                interactive: false,
                similarity: 0.6,
            })
        }
    };

    let buffer = run(run_args).unwrap();

    println!("{:#?}", buffer)
}
