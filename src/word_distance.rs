use crate::word2vec::Repr;

pub fn normalized_distance(a: &Repr, b: &Repr) -> f64 {
    use std::cmp::{max, min};
    let mut count = 0.0;

    for (ai, bi) in a.iter().zip(b.iter()) {
        if *min(ai, bi) == 0 {
            count += 1.0
        } else {
            count += (max(ai, bi) - min(ai, bi)) as f64;
        }
    }

    count / min(a.len(), b.len()) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_zero() {
        assert_eq!(normalized_distance(&vec![1, 1], &vec![1, 1]), 0.0,)
    }

    #[test]
    fn test_distance_one() {
        assert_eq!(normalized_distance(&vec![1, 1], &vec![0, 0]), 1.0)
    }

    #[test]
    fn test_distance_in_between() {
        assert_eq!(
            normalized_distance(&vec![1, 1, 0, 0, 1], &vec![1, 0, 1, 0, 0]),
            0.8,
        )
    }

    #[test]
    fn test_distance_between_zeros_is_one() {
        assert_eq!(normalized_distance(&vec![0, 0], &vec![0, 0]), 1.0)
    }
}
