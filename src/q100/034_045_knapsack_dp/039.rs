use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let mut dp = vec![vec![0usize; 21]; n];
  dp[0][a[0]] = 1;
  for i in 1..n-1 {
    for j in 0..=20 {
      if j >= a[i] {
        dp[i][j] += dp[i-1][j-a[i]];
      }
      if j + a[i] <= 20 {
        dp[i][j] += dp[i-1][j+a[i]];
      }
    }
  }
  println!("{}", dp[n-2][a[n-1]]);
}