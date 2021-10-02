use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    k: usize,
    ab: [(usize, usize); k],
  }
  let mut hm = HashMap::new();
  for &(a, b) in &ab {
    hm.insert(a, b);
  }
  // i日目、i日目のパスタ、i-1日目のパスタ
  let mut dp = vec![vec![vec![0; 4]; 4]; n+1];
  dp[0][0][0] = 1;
  for i in 1..=n {
    for b0 in 1..=3 {
      if let Some(&b) = hm.get(&i) {
        if b0 != b {
          continue;
        }
      }
      for b1 in 0..=3 {
        for b2 in 0..=3 {
          if b0 == b1 && b1 == b2 {
            continue;
          }
          dp[i][b0][b1] += dp[i-1][b1][b2];
          dp[i][b0][b1] %= 10_000;
        }
      }
    }
  }
  let mut ans = 0;
  for b1 in 1..=3 {
    for b2 in 1..=3 {
      ans += dp[n][b1][b2];
      ans %= 10_000;
    }
  }
  println!("{}", ans);
}