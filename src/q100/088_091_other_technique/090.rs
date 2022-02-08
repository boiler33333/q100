use proconio::input;
use std::f64::MAX;

fn main() {
  input! {
    n: usize, m: usize,
    xyr: [(f64, f64, f64); n],
    xy: [(f64, f64); m],
  }
  let mut ans = MAX;
  for &(_, _, r) in &xyr {
    if ans > r {
      ans = r;
    }
  }
  for &(x1, y1, r1) in &xyr {
    for &(x2, y2) in &xy {
      let d = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
      let r2 = d - r1;
      if ans > r2 {
        ans = r2;
      }
    }
  }
  for i in 0..m {
    for j in i+1..m {
      let (x1, y1) = xy[i];
      let (x2, y2) = xy[j];
      let d = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
      let r = d / 2.0;
      if ans > r {
        ans = r;
      }
    }
  }
  println!("{:.6}", ans);
}