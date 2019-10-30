use std::usize::MAX;

use crate::number::linked::LinkedNumber;

pub struct CachedNarcissisticIterator {
    index: LinkedNumber,
    digit: u32,
    digit_mark: usize,
    cache: [usize; 10],
}

impl CachedNarcissisticIterator {
    pub fn new() -> Self {
        CachedNarcissisticIterator {
            index: LinkedNumber::new(0),
            digit: 0,
            digit_mark: 0,
            cache: [0; 10],
        }
    }
}

impl Iterator for CachedNarcissisticIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &value = self.index.value();
            if value == MAX {
                return None;
            }
            if value == self.digit_mark {
                let power = self.digit + 1;
                self.digit = power;
                self.digit_mark = 10_usize.pow(self.digit);
                for i in 0_usize..10 {
                    self.cache[i] = i.pow(power);
                }
            }
            let mut sum = 0;
            let mut node = self.index.node();
            loop {
                match node.next() {
                    Some(next) => {
                        sum = sum + self.cache[*node.value()];
                        node = next;
                    }
                    None => {
                        sum = sum + self.cache[*node.value()];
                        break;
                    }
                }
            }
            self.index.plus_one();
            if sum == value {
                return Some(sum);
            }
        }
    }
}
