//! # MLog Word2Vec
//!
//! In order to efficiently compare text lines, we need to first produce word embeddings.
//! This is a naive implementation; optimised for specific purpose as needed (that is, sometimes not at all).
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub type Word = Vec<char>;
pub type Counter = usize;
pub type Repr = HashMap<Counter, i8>;

#[derive(Debug)]
pub struct Embedding {
    current_idx: Counter,
    words: HashMap<Word, Counter>,
}

impl Embedding {
    pub fn insert(&mut self, line: String) {
        let chars = line.chars().collect::<Vec<char>>();
        let split = chars.split(|c| *c == ' ');

        for word in split {
            let entry = self.words.entry(word.to_vec());
            // TODO or_insert_with(|| ...)
            match entry {
                Entry::Vacant(_) => {
                    self.words.insert(word.to_vec(), self.current_idx);
                    self.current_idx += 1;
                }
                _ => {}
            }
        }
    }

    pub fn repr(&self, line: String) -> Repr {
        let chars = line.chars().collect::<Vec<char>>();
        let split = chars.split(|c| *c == ' ');
        let mut r = Repr::new();

        for word in split {
            let entry = self.words.get(&word.to_vec());
            match entry {
                Some(v) => {
                    r.entry(*v).and_modify(|e| *e += 1).or_insert(1);
                }
                _ => {}
            }
        }

        r
    }

    pub fn new() -> Self {
        Embedding {
            words: HashMap::new(),
            current_idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_line() {
        let line = "words are so meaningless";
        let mut embedding = Embedding::new();
        embedding.insert(line.to_string());

        assert_eq!(
            embedding.repr(line.to_string()),
            [(0, 1), (1, 1), (2, 1), (3, 1)].iter().cloned().collect(),
        )
    }

    #[test]
    fn test_repr_no_intersection() {
        let line = "words are so meaningless";
        let mut embedding = Embedding::new();
        embedding.insert(line.to_string());

        assert_eq!(embedding.repr("nothing matches".to_string()), Repr::new(),)
    }

    #[test]
    fn test_embed_twice() {
        let mut embedding = Embedding::new();
        embedding.insert("words are so meaningless".to_string());
        embedding.insert("nothing matches".to_string());

        assert_eq!(
            embedding.repr("some words matches matches".to_string()),
            [(0, 1), (5, 2)].iter().cloned().collect(),
        )
    }
}
