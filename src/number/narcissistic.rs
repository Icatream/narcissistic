use std::usize::MAX;

use crate::number::listed::{LinkedNumber, Node};

pub struct NarcissisticIterator {
    index: LinkedNumber,
    digit: u32,
    digit_mark: usize,
}

impl NarcissisticIterator {
    pub fn new() -> NarcissisticIterator {
        NarcissisticIterator {
            index: LinkedNumber::new(0),
            digit: 0,
            digit_mark: 0,
        }
    }
}

impl Node {
    fn sum_power_of_each_num(&self, exp: u32, sum: usize) -> usize {
        match self.next {
            Some(ref node) => node.sum_power_of_each_num(exp, sum + self.val.pow(exp)),
            None => sum + self.val.pow(exp),
        }
    }
}

impl Iterator for NarcissisticIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.index.val;
            if value == MAX {
                return None;
            }
            if value == self.digit_mark {
                self.digit = self.digit + 1;
                self.digit_mark = 10_usize.pow(self.digit);
            }
            let sum = self.index.head.sum_power_of_each_num(self.digit, 0);
            self.index.plus_one();
            if sum == value {
                return Some(sum);
            }
        }
    }
}

