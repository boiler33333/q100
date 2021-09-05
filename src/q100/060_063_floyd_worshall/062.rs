use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    h: usize,
    w: usize,
    mut c: [[usize; 10]; 10],
    a: [[i64; w]; h],
  }
  for k in 0..10 {
    for i in 0..10 {
      for j in 0..10 {
        c[i][j] = min(c[i][j], c[i][k] + c[k][j]);
      }
    }
  }
  let mut ans = 0;
  for y in 0..h {
    for x in 0..w {
      if a[y][x] >= 0 {
        let v = a[y][x] as usize;
        ans += c[v][1];
      }
    }
  }
  println!("{}", ans);
}