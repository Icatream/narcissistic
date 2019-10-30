#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use narcissistic::number::narcissistic::NarcissisticIterator;

    use narcissistic::number::cached_narcissistic::CachedNarcissisticIterator;

    #[test]
    fn without_cache() {
        let narcissistic = NarcissisticIterator::new();
        let sys_time = SystemTime::now();
        narcissistic.take(29)
            .for_each(|i| println!("{}", i));
        let difference = SystemTime::now().duration_since(sys_time)
            .expect("SystemTime::duration_since failed");
        println!("narcissistic without cache: {:?}", difference);
    }

    //Vec<[usize;10]> a little bit quick
    //Vec<Vec[usize]>> slower

    #[test]
    fn with_cache() {
        let narcissistic = CachedNarcissisticIterator::new();
        let sys_time = SystemTime::now();
        narcissistic.take(29)
            .for_each(|i| println!("{}", i));
        let difference = SystemTime::now().duration_since(sys_time)
            .expect("SystemTime::duration_since failed");
        println!("narcissistic with cache: {:?}", difference);
    }
}