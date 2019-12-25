use narcissistic::number::n_queens;

fn main() {
  let all = n_queens::calc_queens(5);
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
