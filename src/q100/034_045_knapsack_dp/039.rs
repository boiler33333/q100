use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    a: [i64; n], // a[0] ± ... ± a[n-2] = a[n-1]
  }
  let mut dp: Vec<HashMap<i64, usize>> = vec![HashMap::new(); n];
  dp[0].insert(a[0], 1);
  for i in 1..n-1 {
    for j in 0..=20 {
      if j + a[i] <= 20 {
        let pv = *dp[i-1].get(&j).unwrap_or(&0);
        let cv = *dp[i].get(&(j+a[i])).unwrap_or(&0);
        dp[i].insert(j+a[i], cv + pv);
      }
      if j - a[i] >= 0 {
        let pv = *dp[i-1].get(&j).unwrap_or(&0);
        let cv = *dp[i].get(&(j-a[i])).unwrap_or(&0);
        dp[i].insert(j-a[i], cv + pv);
      }
    }
  }
  println!("{}", dp[n-2].get(&a[n-1]).unwrap_or(&0));
}