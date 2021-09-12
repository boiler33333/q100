use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    d: usize,
    n: usize,
    t: [i64; d],
    abc: [(i64, i64, i64); n],
  }
  let mut available = vec![vec![false; n]; d];
  for i in 0..d {
    for j in 0..n {
      let (a, b, _) = abc[j];
      available[i][j] = a <= t[i] && t[i] <= b;
    }
  }
  let mut dp = vec![vec![0; n]; d];
  for i in 0..d-1 {
    for j in 0..n {
      if !available[i][j] {
        continue;
      }
      let (_, _, cj) = abc[j];
      for k in 0..n {
        if !available[i+1][k] {
          continue;
        }
        let (_, _, ck) = abc[k];
        dp[i+1][k] = max(dp[i+1][k], dp[i][j] + (cj - ck).abs());
      }
    }
  }
  let ans = dp[d-1].iter().max().unwrap();
  println!("{}", ans);
}