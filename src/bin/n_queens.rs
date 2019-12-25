use narcissistic::number::n_queens;
use std::env::args;

fn main() {
  let n = args()
    .skip(1)
    .next()
    .and_then(|s| s.parse::<usize>().ok())
    .unwrap_or(5);

  let all = n_queens::calc_queens(n);
  all.iter().for_each(|queen| {
    println!("{}", queen);
  });
  let unique = n_queens::unique(&all);
  println!("Unique:");
  unique.iter().for_each(|queen| {
    println!("{}", queen);
  });
  println!("All: {}, Unique: {}", all.len(), unique.len());
}
