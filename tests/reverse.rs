#[cfg(test)]
mod tests {

    use narcissistic::number::linked::LinkedNumber;

    #[test]
    fn reverse() {
        let mut num = LinkedNumber::new(210);
        println!("{:?}", num);
        println!("{}", num.calculate_value());
        num.reverse();
        println!("{:?}", num);
        println!("{}", num.value());
    }

    #[test]
    fn reverse_at() {
        let mut num = LinkedNumber::new(54321);
        num.reverse_at(1);
        println!("{:?}", num);
    }
}