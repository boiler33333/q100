use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [[usize; m]; n],
  }
  let mut ans = 0;
  for i in 0..m {
    for j in i+1..m {
      let mut point = 0;
      for k in 0..n {
        point += max(a[k][i], a[k][j]);
      }
      ans = max(ans, point);
    }
  }
  println!("{}", ans);
}