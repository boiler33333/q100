use proconio::input;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    xy: [(f64, f64); n],
  }
  let mut table = vec![vec![0.0; n]; n];
  for v in 0..n {
    for u in 0..n {
      let (x2, y2) = xy[v];
      let (x1, y1) = xy[u];
      table[u][v] = ((x2-x1).powi(2)+(y2-y1).powi(2)).sqrt();
    }
  }
  let mut sum = 0.0;
  for p in (0..n).permutations(n) {
    for i in 1..n {
      sum += table[p[i]][p[i-1]];
    }
  }
  let fac = (1..=n).fold(1, |sum, i| sum * i) as f64;
  let ans = sum / fac;
  println!("{:.10}", ans);
}