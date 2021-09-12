use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let mut dp: Vec<Vec<usize>> = vec![vec![0; 21]; n-1];
  dp[0][a[0]] = 1;
  for i in 1..n-1 {
    for j in 0..21 {
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