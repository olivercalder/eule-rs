/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

use combinatorial::Combinations;

fn main() {
    let limit: i64 = 1000;
    let numbers: Vec<i64> = vec![3, 5];
    let total = Combinations::all(numbers).skip(1).fold(0, |acc, comb| {
        let product: i64 = comb.iter().product();
        let count = (limit - 1) / product;
        let sum = count * (count + 1) / 2;
        let value = sum * product;
        acc + (-1 + 2 * (comb.len() as i64 % 2)) * value
    });
    println!("{}", total);
}
