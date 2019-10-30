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
}