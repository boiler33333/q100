use proconio::input;
use std::collections::VecDeque;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<char>>,
  (sx, sy): (usize, usize),
  (gx, gy): (usize, usize),
) -> usize {
  let mut dist = vec![vec![0; w]; h];
  let mut que = VecDeque::new();
  dist[sy][sx] = 1;
  que.push_back((sx, sy));
  while let Some((ux, uy)) = que.pop_front() {
    for i in 0..4 {
      if uy == 0 && i == 0 || uy == h-1 && i == 2 {
        continue;
      }
      if ux == 0 && i == 3 || ux == w-1 && i == 1 {
        continue;
      }
      let (vx, vy) = match i {
        0 => (ux + 0, uy - 1),
        1 => (ux + 1, uy + 0),
        2 => (ux + 0, uy + 1),
        _ => (ux - 1, uy + 0),
      };
      if table[vy][vx] == '#' {
        continue;
      }
      if dist[vy][vx] == 0 {
        dist[vy][vx] = dist[uy][ux] + 1;
        que.push_back((vx, vy));
      }
    }
  }
  dist[gy][gx]
}

fn main() {
  input! {
    h: usize, w: usize,
    s: [String; h],
  }
  let table: Vec<Vec<char>> = s.iter().map(|x| x.chars().collect()).collect();
  let cnt = bfs(h, w, &table, (0, 0), (w-1, h-1));
  if cnt == 0 {
    println!("-1");
    return;
  }
  let mut ans = h * w - cnt;
  for y in 0..h {
    for x in 0..w {
      if table[y][x] == '#' {
        ans -= 1;
      }
    }
  }
  println!("{}", ans);
}