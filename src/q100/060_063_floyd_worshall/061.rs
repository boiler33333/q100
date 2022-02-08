use proconio::input;
use std::cmp::{max, min};

const INF: usize = 1 << 60;

fn main() {
  input! {
    n: usize, //バス停の数
    m: usize, //路線の数
    abt: [(usize, usize, usize); m], //路線間の移動時間
  }
  let mut dp = vec![vec![INF; n]; n];
  for &(a, b, t) in &abt {
    dp[a-1][b-1] = t;
    dp[b-1][a-1] = t;
  }
  for i in 0..n {
    dp[i][i] = 0;
  }
  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
      }
    }
  }
  let mut ans = INF;
  for i in 0..n {
    let mut tmp = 0;
    for j in 0..n {
      tmp = max(tmp, dp[i][j]);
    }
    ans = min(ans, tmp);
  }
  println!("{}", ans);
}