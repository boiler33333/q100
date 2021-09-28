use proconio::input;
use std::cmp::{max, min};
use std::usize::MAX;

fn main() {
  input! {
    n: usize, k: usize,
    a: [usize; n],
  }
  let mut ans = MAX;
  for state in 0..1<<n {
    if state % 2 == 0 {
      continue;
    }
    let mut cnt = 0;
    for i in 0..n {
      if state & 1 << i > 0 {
        cnt += 1;
      }
    }
    if cnt != k {
      continue;
    }
    let mut h = a.clone();
    let mut cost = 0;
    for i in 1..n {
      if state & 1 << i > 0 {
        if h[i-1] >= h[i] {
          h[i-1] += 1;
          cost += h[i-1] - h[i];
        }
      }
      h[i] = max(h[i-1], h[i]);
    }
    ans = min(ans, cost);
  }
  println!("{}", ans);
}