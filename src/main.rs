use colored::*;
use mlog::*;

use std::collections::LinkedList;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Pattern(String, word2vec::Repr, Color);

#[derive(Debug)]
struct PatternBuffer {
    patterns: LinkedList<Pattern>,
    current_color: Color,
    word_to_vec: Embedding,
}

enum AddOrClosest {
    Added(PatternBuffer, Pattern),
    Closest(PatternBuffer, Pattern, Levenshtein),
}

const MAX_SIZE: usize = 512;

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
        self.word_to_vec.insert(other.to_string());
        let pattern = Pattern(
            other.clone(),
            self.word_to_vec.repr(other.to_string()),
            color,
        );
        self.patterns.push_back(pattern.clone());

        AddOrClosest::Added(self, pattern)
    }

    fn closest(&self, other: &String) -> Option<(f64, Pattern)> {
        let r = self.word_to_vec.repr(other.to_string());

        self.patterns
            .iter()
            .map(|p| (normalized_distance(&p.1, &r), p))
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .map(|(a, p)| (a, p.clone()))
    }

    fn add_or_closest(self, other: &String, target: f64) -> AddOrClosest {
        use AddOrClosest::*;

        match self.closest(&other) {
            None => self.add(other),
            Some((closest_distance, pattern)) => {
                if closest_distance >= target && self.patterns.len() < MAX_SIZE {
                    self.add(other)
                } else {
                    let diff = levenshtein(&pattern.0, other);
                    Closest(self, pattern, diff)
                }
            }
        }
    }
}

fn colorize(diff: &Levenshtein, pattern: &Pattern) -> Vec<ColoredString> {
    use LevenshteinOp::*;

    let mut buf = Vec::with_capacity(diff.len());

    let split_buf = diff.iter().collect::<Vec<&LevenshteinOp>>();
    let split = split_buf.split(|op| match op {
        Keep(x) => *x == ' ',
        Substitute(_, x) => *x == ' ',
        Insert(x) => *x == 'x',
        _ => false,
    });

    for (idx, s) in split.enumerate() {
        let chs = s
            .iter()
            .map(|op| match op {
                Keep(x) => Some(x),
                Substitute(_, x) => Some(x),
                Insert(x) => Some(x),
                _ => None,
            })
            .fold(Vec::with_capacity(s.len()), |mut arr, it| {
                if it.is_some() {
                    arr.push(*(it.unwrap()))
                }
                arr
            });
        let same = s
            .iter()
            .map(|op| match op {
                Keep(_) => true,
                Substitute(_, _) => false,
                Insert(_) => false,
                _ => false,
            })
            .all(|x| x);
        let str = format!("{}", chs.iter().collect::<String>()).color(pattern.2);
        if idx > 0 {
            buf.push(" ".color(pattern.2));
        }
        buf.push(if same { str.dimmed() } else { str });
    }

    buf
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
        word_to_vec: Embedding::new(),
    };

    for line in lines(args.source) {
        let l = line?;
        if l.len() > 0 {
            match buffer.add_or_closest(&l, args.similarity) {
                Closest(b, p, diff) => {
                    buffer = b;
                    if args.interactive {
                        for s in colorize(&diff, &p).iter() {
                            print!("{}", s);
                        }
                    }
                }
                Added(b, p) => {
                    buffer = b;

                    if args.interactive {
                        print!("{}", l.color(p.2));
                    }
                }
            }
        }
        if args.interactive {
            println!("");
        }
    }
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let similarity = 0.4;
    let run_args = match args.len() {
        1 => Args {
            source: SourceOptions::FromStdin,
            interactive: true,
            similarity: similarity,
        },
        _ => Args {
            source: SourceOptions::FromFile(args[1].clone()),
            interactive: false,
            similarity: similarity,
        },
    };

    let buffer = run(run_args).unwrap();

    println!("Found {:#?} patterns", buffer.patterns.len())
}
