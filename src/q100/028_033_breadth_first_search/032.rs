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

#[derive(Copy, Clone)]
struct Wall {
  right: bool,
  bottom: bool,
}

impl Wall {
  fn new() -> Self {
    Wall{ right: true, bottom: true }
  }
}


fn bfs(
  h: usize,
  w: usize,
  wall: &Vec<Vec<Wall>>,
) -> usize {
  let mut dist = vec![vec![0; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  dist[0][0] = 1;
  que.push_back((0, 0));
  while let Some((x, y)) = que.pop_front() {
    for i in 0..4 {
      if y == 0 && i == 2 || y == h-1 && i == 0 {
        continue;
      }
      if x == 0 && i == 3 || x == w-1 && i == 1 {
        continue;
      }
      let (x2, y2) = match i {
        0 => (x+0, y+1),
        1 => (x+1, y+0),
        2 => (x+0, y-1),
        _ => (x-1, y+0),
      };
      let is_wall = match i {
        0 => wall[y][x].bottom,
        1 => wall[y][x].right,
        2 => wall[y-1][x].bottom,
        _ => wall[y][x-1].right,
      };
      if is_wall {
        continue;
      }
      if dist[y2][x2] > 0 {
        continue;
      }
      dist[y2][x2] = dist[y][x] + 1;
      que.push_back((x2, y2));
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
    let mut wall = vec![vec![Wall::new(); w]; h];
    for i in 0..h-1 {
      for j in 0..w-1 {
        wall[i][j].right = read::<usize>() > 0;
      }
      for j in 0..w {
        wall[i][j].bottom = read::<usize>() > 0;
      }
    }
    for j in 0..w-1 {
      wall[h-1][j].right = read::<usize>() > 0;
    }
    let ans = bfs(h, w, &wall);
    println!("{}", ans);
  }
}