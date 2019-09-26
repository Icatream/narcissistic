use std::usize::MAX;

use narcissistic::number::listed::ListedNumber;

pub struct CachedNarcissisticIterator {
    index: ListedNumber,
    digit: u32,
    digit_mark: usize,
    cache: Vec<[usize; 10]>,
}

impl CachedNarcissisticIterator {
    pub fn new() -> CachedNarcissisticIterator {
        CachedNarcissisticIterator {
            index: ListedNumber::new(0),
            digit: 0,
            digit_mark: 0,
            cache: vec![],
        }
    }
}

impl Iterator for CachedNarcissisticIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.index.value();
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
                self.cache.push(arr);
            }
            let cache_index = (self.digit - 1) as usize;
            let cache = self.cache.get(cache_index).unwrap();
            let power_sum_value = self.index.vec().iter()
                .map(|x| cache[*x])
                .sum();
            self.index.plus_one();
            if power_sum_value == value {
                return Some(power_sum_value);
            }
        }
    }
}
