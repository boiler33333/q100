use std::collections::VecDeque;
use std::io::*;
use std::str::FromStr;
use std::usize::MAX;

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
  table: &Vec<Vec<(usize, usize)>>,
) -> usize {
  let mut dist = vec![vec![MAX; w]; h];
  let mut que = VecDeque::new();
  dist[0][0] = 0;
  que.push_back((0, 0));
  while let Some((ux, uy)) = que.pop_front() {
    for i in 0..4 {
      if uy == 0 && i == 0 || uy == h-1 && i == 2 {
        continue;
      }
      if ux == 0 && i == 3 || ux == w-1 && i == 1 {
        continue;
      }
      let is_wall = match i {
        0 => table[uy-1][ux].0 > 0,
        1 => table[uy][ux].1 > 0,
        2 => table[uy][ux].0 > 0,
        _ => table[uy][ux-1].1 > 0,
      };
      if is_wall {
        continue;
      }
      let (vx, vy) = match i {
        0 => (ux + 0, uy - 1),
        1 => (ux + 1, uy + 0),
        2 => (ux + 0, uy + 1),
        3 => (ux - 1, uy + 0),
        _ => unreachable!(),
      };
      if dist[vy][vx] == MAX {
        dist[vy][vx] = dist[uy][ux] + 1;
        que.push_back((vx, vy));
      }
    }
  }
  dist[h-1][w-1]
}

fn main() {
  loop {
    let w: usize = read();
    let h: usize = read();
    if w == 0 && h == 0 {
      break;
    }
    let mut table = vec![vec![(0, 0); w]; h]; //vec![(down, right)]
    for y in 0..h-1 {
      for x in 0..w-1 {
        table[y][x].1 = read();
      }
      for x in 0..w {
        table[y][x].0 = read();
      }
    }
    for x in 0..w-1 {
      table[h-1][x].1 = read();
    }
    let ans = bfs(h, w, &mut table);
    if ans == MAX {
      println!("{}", 0);
    } else {
      println!("{}", ans+1);
    }
  }
}