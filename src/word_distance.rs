use crate::word2vec::Repr;

pub fn normalized_distance(a: &Repr, b: &Repr) -> f64 {
    use std::cmp::{max, min};
    let mut count = 0.0;

    for (ai, ac) in a {
        match b.get(ai) {
            Some(bc) => {
                count += (max(ac, bc) - min(ac, bc)) as f64;
            }
            None => count += *ac as f64,
        }
    }

    for (bi, bc) in a {
        match a.get(bi) {
            Some(_) => {} // Already counted in previous loop
            None => count += *bc as f64,
        }
    }

    let norm = max(a.len(), b.len()) as f64;

    count / norm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_zero() {
        assert_eq!(
            normalized_distance(
                &[(0, 1), (1, 1)].iter().cloned().collect(),
                &[(0, 1), (1, 1)].iter().cloned().collect(),
            ),
            0.0,
        )
    }

    #[test]
    fn test_distance_one() {
        assert_eq!(
            normalized_distance(&[(0, 1), (1, 1)].iter().cloned().collect(), &Repr::new()),
            1.0,
        )
    }

    #[test]
    fn test_distance_in_between() {
        assert_eq!(
            normalized_distance(
                &[(0, 1), (1, 1)].iter().cloned().collect(),
                &[(0, 1), (2, 1)].iter().cloned().collect(),
            ),
            0.5,
        )
    }
}
