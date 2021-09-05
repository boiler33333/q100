use std::cmp::min;
use std::io::*;
use std::str::FromStr;

const INF: i64 = 1 << 60;

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
  let v: usize = read();
  let e: usize = read();
  let mut dp = vec![vec![INF; v]; v];
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let d: i64 = read();
    dp[s][t] = d;
  }
  for i in 0..v {
    dp[i][i] = 0;
  }
  for k in 0..v {
    for i in 0..v {
      for j in 0..v {
        dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
      }
    }
  }
  for i in 0..v {
    if dp[i][i] < 0 {
      println!("NEGATIVE CYCLE");
      return;
    }
  }
  for i in 0..v {
    for j in 0..v {
      if j > 0 {
        print!(" ");
      }
      if dp[i][j] < INF/2 {
        print!("{}", dp[i][j]);
      } else {
        print!("INF");
      }
    }
    println!();
  }
}