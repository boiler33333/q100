use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<bool>>,
  (sx, sy): (usize, usize),
  (gx, gy): (usize, usize),
) -> usize {
  let mut dist = vec![vec![MAX; w]; h];
  let mut que = VecDeque::new();
  dist[sy][sx] = 0;
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
        3 => (ux - 1, uy + 0),
        _ => unreachable!(),
      };
      if table[vy][vx] || dist[vy][vx] < MAX {
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
    h: usize, w: usize, n: usize,
    s: [String; h],
  }
  let mut p = vec![(0, 0); n+1];
  let mut table = vec![vec![false; w]; h];
  for y in 0..h {
    for (x, c) in s[y].chars().enumerate() {
      match c {
        '.' => {},
        'X' => table[y][x] = true,
        'S' => p[0] = (x, y),
        _ => {
          let i = c.to_digit(10).unwrap() as usize;
          p[i] = (x, y);
        }
      }
    }
  } 
  let mut ans = 0;
  for i in 1..=n {
    ans += bfs(h, w, &table, p[i-1], p[i]);
  }
  println!("{}", ans);
}