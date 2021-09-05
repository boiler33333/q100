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
  let mut p = vec![0; n+1];
  for i in 1..n+1 {
    p[i-1] = read();
    p[i] = read();
  }
  let mut dp = vec![vec![0; n+1]; n+1];
  for d in 1..n {
    for i in 1..n {
      let j = i + d;
      if j < n+1 {
        dp[i][j] = MAX;
        for k in i..j {
          dp[i][j] = min(dp[i][j], dp[i][k] + dp[k+1][j] + p[i-1] * p[k] * p[j]);
        } 
      }
    }
  }
  println!("{}", dp[1][n]);
}