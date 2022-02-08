use proconio::input;

fn main() {
  input! {
    n: usize,
    s: String,
  }
  let mut a = vec![0; n+1];
  for (i, c) in s.chars().enumerate() {
    a[i+1] = match c { 'J' => 0, 'O' => 1, 'I' => 2, _ => 3 };
  }
  let mut dp = vec![vec![0; n+1]; 1 << 3];
  // 001: J, 010: O, 100: I
  dp[1<<a[0]][0] = 1;
  for i in 1..=n {
    for member in 0..1<<3 {
      for member2 in 0..1<<3 {
        if member2 & member == 0 {
          continue;
        }
        if member2 & (1 << a[i]) == 0 {
          continue;
        }
        dp[member2][i] += dp[member][i-1];
        dp[member2][i] %= 10007;
      }
    }
  }
  let mut ans = 0;
  for member in 0..1<<3 {
    ans += dp[member][n];
    ans %= 10007;
  }
  println!("{}", ans);
}