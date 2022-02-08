use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<char>>,
  (sx, sy): (usize, usize),
  (gx, gy): (usize, usize),
) -> usize {
  let mut dist = vec![vec![MAX; w]; h];
  let mut que = VecDeque::new();
  dist[sy][sx] = 0;
  que.push_back((sx, sy));
  while let Some((ux, uy)) = que.pop_front() {
    for i in 0..4 {
      let (vx, vy) = match i {
        0 => (ux + 0, uy - 1),
        1 => (ux + 1, uy + 0),
        2 => (ux + 0, uy + 1),
        3 => (ux - 1, uy + 0),
        _ => unreachable!(),
      };
      if table[vy][vx] == '#' || dist[vy][vx] < MAX {
        continue;
      }
      dist[vy][vx] = dist[uy][ux] + 1;
      que.push_back((vx, vy));
    }
  }
  dist[gy][gx]
}

fn main() {
  input! {
    h: usize, w: usize,
    sy: usize, sx: usize,
    gy: usize, gx: usize,
    c: [String; h],
  }
  let c = c.iter().map(|s| s.chars().collect()).collect();
  let ans = bfs(h, w, &c, (sx-1, sy-1), (gx-1, gy-1));
  println!("{}", ans);
}