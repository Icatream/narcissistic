use std::usize::MAX;

use crate::number::listed::ListedNumber;

pub struct NarcissisticIterator {
    index: ListedNumber,
    digit: u32,
    digit_mark: usize,
}

impl NarcissisticIterator {
    pub fn new() -> NarcissisticIterator {
        NarcissisticIterator {
            index: ListedNumber::new(0),
            digit: 0,
            digit_mark: 0,
        }
    }
}

impl Iterator for NarcissisticIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.index.value();
            if value == MAX {
                return None;
            }
            if value == self.digit_mark {
                self.digit = self.digit + 1;
                self.digit_mark = 10_usize.pow(self.digit);
            }
            let power_sum_value = self.index.vec.iter()
                .map(|x| (*x).pow(self.digit))
                .sum();
            self.index.plus_one();
            if power_sum_value == value {
                return Some(power_sum_value);
            }
        }
    }
}

