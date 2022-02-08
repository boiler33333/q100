use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let mut acc = a.clone();
  acc.insert(0, 0);
  for i in 1..=n {
    acc[i] += acc[i-1];
  }
  for d in 1..=n {
    let mut ans = 0;
    for i in 0..=n-d {
      ans = max(ans, acc[i+d] - acc[i]);
    }
    println!("{}", ans);
  }
}