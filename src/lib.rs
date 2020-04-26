#[derive(Debug, Clone, PartialEq)]
pub enum LevenshteinOp {
    Keep(char),
    Insert(char),
    Delete(char),
    Substitute(char, char),
}

pub type Levenshtein = Vec<LevenshteinOp>;

/// Calculates the minimum number of insertions, deletions, and substitutions
/// required to change one sequence into the other.
pub fn levenshtein(a: &str, b: &str) -> Vec<LevenshteinOp> {
    use std::cmp::{max, min};
    use LevenshteinOp::*;

    if a.len() == 0 {
        return b.chars().map(|x| Insert(x)).collect();
    }

    let mut oracle = vec![vec![(0, Keep('0')); b.len() + 1]; a.len() + 1];

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for (i, a_elem) in a_chars.iter().enumerate() {
        oracle[i][0] = (i, Delete(*a_elem));
    }
    for (j, b_elem) in b_chars.iter().enumerate() {
        oracle[0][j] = (j, Insert(*b_elem));
    }

    for (i, a_elem) in a_chars.iter().enumerate() {
        for (j, b_elem) in b_chars.iter().enumerate() {
            let cost = if a_elem == b_elem { 0 } else { 1 };
            let when_inserted = oracle[i + 1][j].0 + 1;
            let when_deleted = oracle[i][j + 1].0 + 1;
            let when_shortened = oracle[i][j].0 + cost;

            let op_cost = min(min(when_inserted, when_deleted), when_shortened);
            let op = match op_cost {
                _ if op_cost == when_shortened => {
                    if cost == 0 {
                        Keep(*b_elem)
                    } else {
                        Substitute(*a_elem, *b_elem)
                    }
                }
                _ if op_cost == when_inserted => Insert(*b_elem),
                _ if op_cost == when_deleted => Delete(*a_elem),
                _ => {
                    assert!(false, "op_cost didn't match any option?");
                    Keep(*b_elem)
                }
            };
            oracle[i + 1][j + 1] = (op_cost, op);
        }
    }

    let mut operations = Vec::with_capacity(max(a.len(), b.len()));

    let mut x = a.len();
    let mut y = b.len();

    while x > 0 && y > 0 {
        let (_, op) = &oracle[x][y];
        match op {
            Keep(_) | Substitute(_, _) => {
                y -= 1;
                x -= 1;
            }
            Insert(_) => y -= 1,
            Delete(_) => x -= 1,
        }
        operations.push(op.clone());
    }

    while x > 0 {
        operations.push(Delete(a_chars[x - 1]));
        x -= 1;
    }

    while y > 0 {
        operations.push(Insert(b_chars[y - 1]));
        y -= 1;
    }

    operations.reverse();

    operations
}

fn count_nonkeep(l: &Levenshtein) -> usize {
    l.iter()
        .filter(|op| match op {
            LevenshteinOp::Keep(_) => false,
            _ => true,
        })
        .count()
}

pub fn normalize(a: &str, b: &str, l: &Levenshtein) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    1.0 - (count_nonkeep(l) as f64) / (a.chars().count().max(b.chars().count()) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_delete() {
        use LevenshteinOp::*;

        assert_eq!(
            levenshtein("fafa", "afa"),
            vec![Delete('f'), Keep('a'), Keep('f'), Keep('a'),]
        );
    }

    #[test]
    fn test_simple_insert() {
        use LevenshteinOp::*;

        assert_eq!(
            levenshtein("afa", "fafa"),
            vec![Insert('f'), Keep('a'), Keep('f'), Keep('a'),]
        );
    }

    #[test]
    fn test_wiki_example() {
        use LevenshteinOp::*;

        assert_eq!(
            levenshtein("kitten", "sitting"),
            vec![
                Substitute('k', 's'),
                Keep('i'),
                Keep('t'),
                Keep('t'),
                Substitute('e', 'i'),
                Keep('n'),
                Insert('g')
            ]
        );
    }

    #[test]
    fn test_all_delete() {
        use LevenshteinOp::*;

        assert_eq!(
            levenshtein("fafa", ""),
            vec![Delete('f'), Delete('a'), Delete('f'), Delete('a'),]
        );
    }

    #[test]
    fn test_all_insert() {
        use LevenshteinOp::*;

        assert_eq!(
            levenshtein("", "fafa"),
            vec![Insert('f'), Insert('a'), Insert('f'), Insert('a'),]
        );
    }
}
