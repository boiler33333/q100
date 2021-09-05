use std::cmp::max;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn main() {
  let n: usize = read();
  let m: usize = read();
  let mut v = vec![0; n];
  let mut w = vec![0; n];
  for i in 0..n {
    v[i] = read();
    w[i] = read();
  }
  let mut dp = vec![vec![0; m+1]; n+1];
  for i in 0..n {
    for j in 0..=m {
      if j >= w[i] {
        dp[i+1][j] = max(dp[i+1][j], dp[i][j - w[i]] + v[i]);
      }
      dp[i+1][j] = max(dp[i+1][j], dp[i][j]);
    }
  }
  println!("{}", dp[n][m])
}