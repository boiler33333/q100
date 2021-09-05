use proconio::input;

fn main() {
  input! {
    mut a: usize,
    mut b: usize,
    mut k: usize,
  }
  if a > k {
    println!("{} {}", a - k, b);
  } else if (a + b) > k {
    println!("0 {}", (a + b) - k);
  } else {
    println!("0 0");
  }
}