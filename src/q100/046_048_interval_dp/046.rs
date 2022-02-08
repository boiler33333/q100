use std::cmp::min;
use std::io::*;
use std::str::FromStr;
use std::i64::MAX;

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
  let mut p = vec![0; n+1];
  for i in 1..=n {
    p[i-1] = read();
    p[i] = read();
  }
  let mut dp = vec![vec![0; n+1]; n+1];
  for d in 1..n+1 {
    for l in 1..n+1-d {
      let r = l + d;
      dp[l][r] = MAX;
      for m in l..r {
        dp[l][r] = min(dp[l][r], dp[l][m] + dp[m+1][r] + p[l-1] * p[m] * p[r]);
      }
    }
  }
  println!("{}", dp[1][n]);
}