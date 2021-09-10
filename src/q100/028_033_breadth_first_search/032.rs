use std::collections::VecDeque;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn bfs(
  h: usize,
  w: usize,
  wall: &Vec<Vec<(bool, bool)>>,
  sy: usize,
  sx: usize,
  gy: usize,
  gx: usize,
) -> usize {
  let mut dist = vec![vec![0; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
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
      let is_wall = match i {
        0 => wall[uy-1][ux].1,
        1 => wall[uy][ux].0,
        2 => wall[uy][ux].1,
        _ => wall[uy][ux-1].0,
      };
      if is_wall {
        continue;
      }
      let (vx, vy) = match i {
        0 => (ux + 0, uy - 1),
        1 => (ux + 1, uy + 0),
        2 => (ux + 0, uy + 1),
        _ => (ux - 1, uy + 0),
      };
      if dist[vy][vx] == 0 {
        dist[vy][vx] = dist[uy][ux] + 1;
        que.push_back((vx, vy));
      }
    }
  }
  dist[gy][gx]
}

fn main() {
  loop {
    let w: usize = read();
    let h: usize = read();
    if w == 0 && h == 0 {
      break;
    }
    let mut wall = vec![vec![(false, false); w]; h]; //右、下方向が壁かどうか
    for y in 0..h-1 {
      for x in 0..w-1 {
        let v: usize = read();
        wall[y][x].0 = v == 1;
      }
      for x in 0..w {
        let v: usize = read();
        wall[y][x].1 = v == 1;
      }
    }
    for x in 0..w-1 {
      let v: usize = read();
      wall[h-1][x].0 = v == 1;
    }
    let ans = bfs(h, w, &wall, 0, 0, h-1, w-1);
    println!("{}", ans);
  }
}