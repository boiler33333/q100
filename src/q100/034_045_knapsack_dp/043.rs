use proconio::input;
use std::cmp::min;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    s: [String; 5],
  }
  let mut table = vec![vec![0; n]; 5];
  for y in 0..5 {
    for (x, c) in s[y].chars().enumerate() {
      table[y][x] = match c {
        'R' => 0,
        'B' => 1,
        'W' => 2,
        _ => 3,
      };
    }
  }
  // dp[column][color]
  let mut dp = vec![vec![MAX; 3]; n+1];
  for i in 0..3 {
    dp[0][i] = 0;
  }
  for x in 0..n {
    for i in 0..3 {
      let mut cnt = 0;
      for y in 0..5 {
        if table[y][x] != i {
          cnt += 1;
        }
      }
      for j in 0..3 {
        if i != j {
          dp[x+1][j] = min(dp[x+1][j], dp[x][i] + cnt);
        }
      }
    }
  }
  let ans = dp[n].iter().min().unwrap();
  println!("{}", ans);
}