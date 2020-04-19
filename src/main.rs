extern crate strsim;

use std::collections::LinkedList;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use strsim::normalized_levenshtein;

#[derive(Debug)]
struct Pattern(String);

#[derive(Debug)]
struct PatternBuffer {
    patterns: LinkedList<Pattern>,
}

impl PatternBuffer {
    fn try_add(&mut self, other: String, target: f64) {
        let max = self
            .patterns
            .iter()
            .map(|p| normalized_levenshtein(&p.0, &other))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(target);
        if max <= target {
            self.patterns.push_back(Pattern(other));
        }
    }
}

struct Args<'a> {
    input: Box<dyn BufRead + 'a>,
    interactive: bool,
    similarity: f64,
}

fn run<'a>(args: Args<'a>) -> Result<PatternBuffer, io::Error> {
    let mut buffer = PatternBuffer {
        patterns: LinkedList::new(),
    };

    for line in args.input.lines() {
        let l = line?;
        if args.interactive {
            println!("{}", l)
        }
        if l.len() > 0 {
            buffer.try_add(l, args.similarity);
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
