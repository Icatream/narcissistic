use std::env::args;

use crate::number::cached_narcissistic::CachedNarcissisticIterator;

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
    let narcissistic = CachedNarcissisticIterator::new();
    let action = |i: usize| println!("{}", i);
    match n {
        Some(n) => narcissistic.take(n)
            .for_each(action),
        None => narcissistic.for_each(action)
    }
}