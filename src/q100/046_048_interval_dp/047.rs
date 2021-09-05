use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let a = [a.clone(), a].concat();
  let mut dp = vec![vec![0; n]; n];
  for i in 0..n {
    dp[i][i] = a[i];
  }
  for d in 0..n {
    for l in 0..n {
      let r = (l + d) % n;
      let l2 = (n + l - 1) % n;
      let r2 = (r + 1) % n;
      if d % 2 == 0 { // IOI
        if a[l2] < a[r2] {
          dp[l][r2] = max(dp[l][r2], dp[l][r]);
        } else {
          dp[l2][r] = max(dp[l2][r], dp[l][r]);
        }
      } else { // JOI
        dp[l][r2] = max(dp[l][r2], dp[l][r] + a[r2]);
        dp[l2][r] = max(dp[l2][r], dp[l][r] + a[l2]);
      }
    }
  }
  let mut ans = 0;
  for l in 0..n {
    let r = (l + n - 1) % n;
    ans = max(ans, dp[l][r]);
  }
  println!("{}", ans);
}