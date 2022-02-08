use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize,
    p: [(i64, i64); n],
    m: usize,
    q: [(i64, i64); m],
  };
  let hs: HashSet<(i64, i64)> = q.iter().cloned().collect();
  for &(x1, y1) in &p {
    for &(x2, y2) in &q {
      let dx = x2 - x1;
      let dy = y2 - y1;
      let mut ok = true;
      for &(x, y) in &p {
        if !hs.contains(&(x+dx, y+dy)) {
          ok = false;
        }
      }
      if ok {
        println!("{} {}", dx, dy);
        return;
      }
    }
  }
}