use std::env::args;

use crate::number::narcissistic::NarcissisticIterator;

pub mod number;

fn main() {
    let n = args()
        .skip(1)
        .next()
        .and_then(|s| {
            match s.parse::<usize>() {
                Ok(i) => Some(i),
                Err(_) => None
            }
        });
    let narcissistic = NarcissisticIterator::new();
    let action = |i: usize| println!("{}", i);
    match n {
        Some(n) => narcissistic.take(n)
            .for_each(action),
        None => narcissistic.take(27).for_each(action)
    }
}