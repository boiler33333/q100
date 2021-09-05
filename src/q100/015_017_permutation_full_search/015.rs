use proconio::input;
use itertools::Itertools;

fn dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn factorial(k: usize) -> usize {
  (2..=k).fold(1, |sum, i| sum * i)
}

fn main() {
  input! {
    n: usize,
    xy: [(f64, f64); n],
  }
  let mut sum = 0.0;
  for p in (0..n).permutations(n) {
    let mut d = 0.0;
    for i in 0..n-1 {
      let u = p[i];
      let v = p[i+1];
      d += dist(xy[u], xy[v]);
    }
    sum += d;
  }
  let m = factorial(n) as f64;
  let ans = sum / m;
  println!("{:.10}", ans);
}