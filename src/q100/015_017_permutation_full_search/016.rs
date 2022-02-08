use proconio::input;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    p: [usize; n],
    q: [usize; n],
  }
  let mut a = 0;
  let mut b = 0;
  for (i, r) in (1..=n).permutations(n).enumerate() {
    if p == r {
      a = i;
    }
    if q == r {
      b = i;
    }
  }
  let ans = if a > b { a - b } else { b - a };
  println!("{}", ans);
}