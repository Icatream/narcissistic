use std::env::args;

use crate::number::cached_narcissistic::CachedNarcissisticIterator;

pub mod number;

fn main() {
    let narcissistic = CachedNarcissisticIterator::new();
    let n = args().skip(1).next().and_then(|s| s.parse::<usize>().ok());
    let action = |i: usize| println!("{}", i);
    match n {
        Some(n) => narcissistic.take(n).for_each(action),
        None => narcissistic.for_each(action),
    }
}
