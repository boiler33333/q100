use std::cmp::min;
use std::io::*;
use std::str::FromStr;
use std::usize::MAX;

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
  let mut c = vec![0; n];
  for i in 0..m {
    c[i] = read();
  }
  let mut dp = vec![vec![MAX; n+1]; m+1];
  dp[0][0] = 0;
  for j in 0..m {
    for i in 0..=n {
      if i >= c[j] {
        dp[j+1][i] = min(dp[j+1][i], dp[j+1][i-c[j]] + 1);
      }
      dp[j+1][i] = min(dp[j+1][i], dp[j][i]);
    }
  }
  println!("{}", dp[m][n]);
}