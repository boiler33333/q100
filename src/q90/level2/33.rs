use proconio::input;

fn main() {
  input! { h: usize, w: usize }
  if h == 1 || w == 1 {
    println!("{}", h * w);
  } else {
    println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
  }
}