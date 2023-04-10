/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

use combinatorial::Combinations;

/// Computes the sum of all multiples of the given numbers below the specified limit.
///
/// Assumes that `numbers` does not contain duplicate entries that no entry is a multiple of,
/// any other entry, and that all entries are positive.
fn sum_of_multiples(numbers: Vec<i64>, limit: i64) -> i64 {
    Combinations::all(numbers).skip(1).fold(0, |acc, comb| {
        let product: i64 = comb.iter().product();
        let count = (limit - 1) / product;
        let sum = count * (count + 1) / 2;
        let value = sum * product;
        acc + (-1 + 2 * (comb.len() % 2) as i64) * value
    })
}

/// Removes duplicate numbers and entries which are multiples of another entry and makes all
/// entries positive.
fn remove_duplicates_multiples(numbers: Vec<i64>) -> Vec<i64> {
    let mut clean: Vec<i64> = numbers.iter().map(|x| x * (!((x >> 62) & 2) + 2)).collect();
    clean.sort_unstable();
    clean.dedup();
    clean.iter().fold(Vec::new(), |mut acc, number| {
        if !acc.iter().any(|x| number % x == 0) {
            acc.push(*number);
        }
        acc
    })
}

/// Computes the sum of all multiples of the given numbers below the specified limit.
///
/// Computes the correct answer regardless of whether `numbers` contains duplicate entries,
/// negative numbers, or entries which are multiples of any other entry.
fn sum_of_multiples_safe(numbers: Vec<i64>, limit: i64) -> i64 {
    let clean = remove_duplicates_multiples(numbers);
    sum_of_multiples(clean, limit)
}

fn main() {
    let numbers: Vec<i64> = vec![3, 5];
    let limit: i64 = 1000;
    let total = sum_of_multiples(numbers, limit);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let numbers = vec![3, 5];
        let limit = 1000;
        assert_eq!(sum_of_multiples(numbers, limit), 233168);
    }

    #[test]
    fn test_more_numbers() {
        let numbers = vec![2, 3, 5, 7];
        let limit = 1000;
        assert_eq!(sum_of_multiples(numbers, limit), 385788);
    }

    #[test]
    fn test_numbers_with_multiples() {
        let numbers = vec![3, 5, 6];
        let limit = 1000;
        assert_eq!(sum_of_multiples_safe(numbers, limit), 233168);
    }

    #[test]
    fn test_numbers_with_negatives() {
        let numbers = vec![2, -3, 5, 7];
        let limit = 1000;
        assert_eq!(sum_of_multiples_safe(numbers, limit), 385788);
    }
}
