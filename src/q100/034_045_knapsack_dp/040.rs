use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    ab: [(usize, usize); k],
  }
  let mut pasta = vec![0; n];
  for &(a, b) in &ab {
    pasta[a-1] = b;
  }
  // i日目、i-1日目のパスタ、i-2日目のパスタ
  let mut dp = vec![vec![vec![0; 4]; 4]; n+1];
  dp[0][0][0] = 1;
  for i in 0..n {
    for b in 1..4 {
      if pasta[i] > 0 && pasta[i] != b {
        continue;
      }
      for b1 in 0..4 {
        for b2 in 0..4 {
          if b == b1 && b1 == b2 {
            continue;
          }
          dp[i+1][b][b1] += dp[i][b1][b2];
          dp[i+1][b][b1] %= 10000;
        }
      }
    }
  }
  let mut ans = 0;
  for i in 1..4 {
    for j in 1..4 {
      ans += dp[n][i][j];
      ans %= 10000;
    }
  }
  println!("{}", ans);
}