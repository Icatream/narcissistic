use narcissistic::number::n_queens;

fn main() {
  let vec = n_queens::calc_queens(6);
  vec.iter().for_each(|queen| {
    println!("{}", queen);
  });
}
