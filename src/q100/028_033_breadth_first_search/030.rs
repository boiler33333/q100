use proconio::input;
use std::collections::VecDeque;

fn bfs(
  h: usize,
  w: usize,
  wall: &Vec<Vec<bool>>,
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
      if wall[vy][vx] || dist[vy][vx] != None {
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
    h: usize, w: usize, n: usize,
    table: [String; h],
  }
  let mut wall: Vec<Vec<bool>> = vec![vec![false; w]; h];
  let mut point: Vec<(usize, usize)> = vec![(0, 0); n+1];
  for y in 0..h {
    for (x, c) in table[y].chars().enumerate() {
      match c {
        '.' => {},
        'X' => wall[y][x] = true,
        'S' => point[0] = (x, y),
        num => {
          let i = num.to_digit(10).unwrap() as usize;
          point[i] = (x, y);
        },
      }
    }
  }
  let mut ans = 0;
  for i in 0..n {
    let (sx, sy) = point[i];
    let (gx, gy) = point[i+1];
    let d = bfs(h, w, &wall, sy, sx, gy, gx);
    ans += d.unwrap_or(0);
  }
  println!("{}", ans);
}