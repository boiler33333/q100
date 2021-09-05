use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
    m: usize,
    p: [(i64, i64); m],
    n: usize,
    q: [(i64, i64); n],
  }
  let (dx, dy) = solve(&p, &q);
  println!("{} {}", dx, dy);
}

fn solve(
  p: &Vec<(i64, i64)>,
  q: &Vec<(i64, i64)>,
) -> (i64, i64) {
  let hs: HashSet::<(i64, i64)> = q.iter().cloned().collect();
  for &(px, py) in p {
    for &(qx, qy) in q {
      let dx = qx - px;
      let dy = qy - py;
      let mut find = true;
      for &(x, y) in p {
        if !hs.contains(&(x + dx, y + dy)) {
          find = false;
        }
      }
      if find {
        return (dx, dy);
      }
    }
  }
  (0, 0)
}