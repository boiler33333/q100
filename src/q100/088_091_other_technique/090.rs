use proconio::input;
use std::f64::MAX;

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn main() {
  input! {
    n: usize, m: usize,
    c1: [(f64, f64, f64); n],
    c2: [(f64, f64); m],
  }
  let mut ans = MAX;
  for &(_, _, r1) in &c1 {
    if ans > r1 {
      ans = r1;
    }
  }
  //cmp c1 c2
  for &(x1, y1, r1) in &c1 {
    for &(x2, y2) in &c2 {
      let r2 = dist(x1, y1, x2, y2) - r1;
      if ans > r2 {
        ans = r2;
      }
    }
  }
  //cmp c2 c2
  for i in 0..m {
    let (x1, y1) = c2[i];
    for j in 0..m {
      if i == j {
        continue;
      }
      let (x2, y2) = c2[j];
      let d = dist(x1, y1, x2, y2);
      let r = d / 2.0;
      if ans > r {
        ans = r;
      }
    }
  }
  println!("{:.6}", ans);
}