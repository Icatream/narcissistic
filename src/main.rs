use std::env::args;

use crate::number::cached_narcissistic::CachedNarcissisticIterator;
use narcissistic::number::linked::Node;

pub mod number;

fn main() {
    let node = Node::new(43210);
    println!("{:?}", node);
    let r_node = node._reverse_at(2);
    println!("{:?}", r_node);

   /* let n = args()
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
    }*/
}