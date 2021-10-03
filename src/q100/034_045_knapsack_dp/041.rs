use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    d: usize,      //服の計画を立てる日数
    n: usize,      //服の種類の数
    t: [usize; d], //i日目の最高気温
    abc: [(usize, usize, i64); n], //服
  }
  let mut clothes = vec![vec![]; d]; //i日目に利用できる服
  for i in 0..d {
    for j in 0..n {
      let &(a, b , c) = &abc[j];
      if a <= t[i] && t[i] <= b {
        clothes[i].push((j, c));
      }
    }
  }
  let mut dp = vec![vec![0; n+1]; d+1];
  for i in 0..d-1 {
    for &(j0, c0) in &clothes[i] {
      for &(j1, c1) in &clothes[i+1] {
        dp[i+1][j1] = max(dp[i+1][j1], dp[i][j0] + (c1 - c0).abs());
      }
    }
  }
  println!("{}", dp[d-1].iter().max().unwrap());
}