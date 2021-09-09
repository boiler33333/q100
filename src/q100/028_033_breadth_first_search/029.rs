use proconio::input;
use std::collections::VecDeque;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<char>>,
  sy: usize,
  sx: usize,
  gy: usize,
  gx: usize,
) -> Option<usize> {
  let mut dist = vec![vec![None; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  dist[sy][sx] = Some(0);
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
      if table[vy][vx] == '#' || dist[vy][vx] != None {
        continue;
      }
      let d = dist[uy][ux].unwrap();
      dist[vy][vx] = Some(d+1);
      que.push_back((vx, vy));
    }
  }
  dist[gy][gx]
}

fn main() {
  input! {
    r: usize, c: usize,
    sy: usize, sx: usize,
    gy: usize, gx: usize,
    table: [String; r],
  }
  let table: Vec<Vec<char>> = table.iter().map(|s| s.chars().collect()).collect();
  let ans = bfs(r, c, &table, sy-1, sx-1, gy-1, gx-1);
  println!("{}", ans.unwrap());
}