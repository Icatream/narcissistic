use narcissistic::number::n_queens;

fn main() {
  let vec = n_queens::calc_queens(5);
  vec.iter().for_each(|queen| {
    println!("{}", queen);
  });
  let fudamental_vec = n_queens::fundamental(&vec);
  println!("Fudamentaled:");
  fudamental_vec.iter().for_each(|queen| {
    println!("{}", queen);
  });
  println!("Size {}, {}", vec.len(), fudamental_vec.len());
}
