use proconio::input;
use std::cmp::{max, min};
use std::i64::MAX;

fn main() {
  input! {
    a: i64, b: i64, c: i64,
    x: i64, y: i64,
  }
  let n = max(x, y);
  let mut ans = MAX;
  for i in 0..=n {
    let v = c * 2 * i + a * max(0, x-i) + b * max(0, y-i);
    ans = min(ans, v);
  }
  println!("{}", ans);
}