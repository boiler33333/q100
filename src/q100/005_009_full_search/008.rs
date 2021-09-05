use proconio::input;
use std::cmp::min;
use std::i64::MAX;

fn main() {
  input! {
    n: usize,
    ab: [(i64, i64); n],
  }
  let ans = solve(&ab);
  println!("{}", ans);
}

fn solve(ab: &Vec<(i64, i64)>) -> i64 {
  let mut res = MAX;
  let mut p = vec![];
  for &(a, b) in ab {
    p.push(a);
    p.push(b);
  }
  for &s in &p {
    for &g in &p {
      let mut d = 0;
      for &(a, b) in ab {
        d += (a - s).abs() + (b - a).abs() + (g - b).abs();
      }
      res = min(res, d)
    }
  }
  res
}