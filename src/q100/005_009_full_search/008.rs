use proconio::input;
use std::collections::HashSet;
use std::cmp::min;
use std::i64::MAX;

fn main() {
  input! {
    n: usize,
    ab: [(i64, i64); n],
  };
  let mut hs = HashSet::new();
  for &(a, b) in &ab {
    hs.insert(a);
    hs.insert(b);
  }
  let mut ans = MAX;
  for &s in &hs {
    for &t in & hs {
      let mut sec = 0;
      for &(a, b) in &ab {
        sec += (t - b).abs() + (b - a).abs() + (a - s).abs();
      }
      ans = min(ans, sec);
    }
  }
  println!("{}", ans);
}