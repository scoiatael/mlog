extern crate strsim;

use std::collections::LinkedList;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
struct Pattern(String);

#[derive(Debug)]
struct PatternBuffer {
    patterns: LinkedList<Pattern>,
}

enum AddOrClosest {
    Added(PatternBuffer),
    Closest(PatternBuffer, Pattern),
}

impl PatternBuffer {
    fn add(mut self, other: &String) -> AddOrClosest {
        self.patterns.push_back(Pattern(other.clone()));

        AddOrClosest::Added(self)
    }

    fn closest(&self, other: &String) -> Option<(f64, Pattern)> {
        match self
            .patterns
            .iter()
            .map(|p| (normalized_levenshtein(&p.0, &other), p))
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        {
            None => None,
            Some((max, p)) => Some((max, (*p).clone())),
        }
    }

    fn add_or_closest(self, other: &String, target: f64) -> AddOrClosest {
        use AddOrClosest::*;

        match self.closest(&other) {
            None => self.add(other),
            Some((max, best)) => {
                if max <= target {
                    self.add(other)
                } else {
                    Closest(self, best)
                }
            }
        }
    }
}

struct Args<'a> {
    input: Box<dyn BufRead + 'a>,
    interactive: bool,
    similarity: f64,
}

fn run<'a>(args: Args<'a>) -> Result<PatternBuffer, io::Error> {
    use AddOrClosest::*;

    let mut buffer = PatternBuffer {
        patterns: LinkedList::new(),
    };

    for line in args.input.lines() {
        let l = line?;
        if l.len() > 0 {
            match buffer.add_or_closest(&l, args.similarity) {
                Closest(b, var) => {
                    buffer = b;
                    if args.interactive {
                        println!("{} => {}", l, var.0)
                    }
                }
                Added(b) => {
                    buffer = b;

                    if args.interactive {
                        println!("{}", l)
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
                input: Box::new(BufReader::new(io::stdin())),
                interactive: true,
                similarity: 0.6,
            })
        }
        _ => {
            (Args {
                input: Box::new(BufReader::new(File::open(args[1].clone()).unwrap())),
                interactive: false,
                similarity: 0.6,
            })
        }
    };

    let buffer = run(run_args).unwrap();

    println!("{:#?}", buffer)
}
