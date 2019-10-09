use std::usize::MAX;

use narcissistic::number::listed::LinkedNumber;

pub struct CachedNarcissisticIterator {
    index: LinkedNumber,
    digit: u32,
    digit_mark: usize,
    cache: [usize; 10],
}

impl CachedNarcissisticIterator {
    pub fn new() -> CachedNarcissisticIterator {
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
            let value = self.index.val;
            if value == MAX {
                return None;
            }
            if value == self.digit_mark {
                let power = self.digit + 1;
                self.digit = power;
                self.digit_mark = 10_usize.pow(self.digit);
                let mut arr = [0_usize; 10];
                for i in 0_usize..10 {
                    arr[i] = i.pow(power);
                }
                self.cache = arr;
            }
            let mut sum = 0;
            let mut node = &self.index.head;
            loop {
                match node.next {
                    Some(ref next) => {
                        sum = sum + self.cache[node.val];
                        node = next;
                    }
                    None => {
                        sum = sum + self.cache[node.val];
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
