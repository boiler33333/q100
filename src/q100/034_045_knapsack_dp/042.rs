use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: usize,
    m: usize,
    d: [i64; n],
    c: [i64; m],
  }
  let mut dp = vec![vec![1_000_000_000; m+1]; n+1];
  for j in 0..m {
    dp[0][j] = 0;
  }
  for i in 0..n {
    for j in 0..m {
      dp[i+1][j+1] = min(dp[i+1][j], dp[i][j] + d[i] * c[j]);
    }  
  }
  println!("{}", dp[n][m]);
}