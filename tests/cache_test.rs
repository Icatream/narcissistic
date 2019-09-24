mod cached_narcissistic;

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use narcissistic::number::narcissistic::NarcissisticIterator;

    use crate::cached_narcissistic::CachedNarcissisticIterator;

    #[test]
    fn without_cache() {
        let narcissistic = NarcissisticIterator::new();
        let sys_time = SystemTime::now();
        narcissistic.take(27)
            .for_each(|i| println!("{}", i));
        let difference = SystemTime::now().duration_since(sys_time)
            .expect("SystemTime::duration_since failed");
        println!("narcissistic without cache: {:?}", difference);
    }

    #[test]
    fn with_cache() {
        let narcissistic = CachedNarcissisticIterator::new();
        let sys_time = SystemTime::now();
        narcissistic.take(27)
            .for_each(|i| println!("{}", i));
        let difference = SystemTime::now().duration_since(sys_time)
            .expect("SystemTime::duration_since failed");
        println!("narcissistic with cache: {:?}", difference);
    }
}