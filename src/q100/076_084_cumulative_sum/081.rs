use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n],
  }
  let m = 1_000_001;
  let mut acc: Vec<i64> = vec![0; m+1];
  for (a, b) in ab {
    acc[a] += 1;
    acc[b+1] -= 1;
  }
  for i in 0..m {
    acc[i+1] += acc[i];
  }
  let mut ans = 0;
  for i in 0..=m {
    ans = max(ans, acc[i]);
  }
  println!("{}", ans);
}