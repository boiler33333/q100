use proconio::input;
use std::cmp::{max, min};
use std::usize::MAX;

fn main() {
  input! {
    n: usize, k: usize,
    a: [usize; n],
  }
  let mut ans = MAX;
  for status in 0..1<<n {
    if status & 1 << 0 == 0 {
      continue;
    }
    let mut cnt = 0;
    for i in 0..n {
      if status & 1 << i != 0 {
        cnt += 1;
      }
    }
    if cnt != k {
      continue;
    }
    let mut cost = 0;
    let mut h = a[0];
    for i in 1..n {
      if status & 1 << i != 0 {
        if a[i] <= h {
          h += 1;
          cost += h - a[i];
        }
      }
      h = max(h, a[i]);
    }
    ans = min(ans, cost);
  }
  println!("{}", ans);
}