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

fn erase(h: usize, w: usize, g: &mut Vec<Vec<usize>>, y: usize, x: usize) {
  for i in 0..9 {
    if y == 0 && i <= 2 || y == h-1 && 6 <= i {
      continue;
    }
    if x == 0 && i % 3 == 0 || x == w-1 && i % 3 == 2 {
      continue;
    }
    let (x2 ,y2) = match i {
      0 => (x - 1, y - 1),
      1 => (x + 0, y - 1),
      2 => (x + 1, y - 1),
      3 => (x - 1, y + 0),
      4 => (x + 0, y + 0),
      5 => (x + 1, y + 0),
      6 => (x - 1, y + 1),
      7 => (x + 0, y + 1),
      _ => (x + 1, y + 1),
    };
    if g[y2][x2] == 0 {
      continue;
    }
    g[y2][x2] = 0;
    erase(h, w, g, y2, x2);
  }
}

fn main() {
  loop {
    let w: usize = read();
    let h: usize = read();
    if w == 0 && h == 0 {
      break;
    }
    let mut g = vec![vec![0; w]; h];
    for i in 0..h {
      for j in 0..w {
        g[i][j] = read();
      }
    }
    let mut ans = 0;
    for y in 0..h {
      for x in 0..w {
        if g[y][x] == 1 {
          ans += 1;
          erase(h, w, &mut g, y, x);
        }
      }
    }
    println!("{}", ans);
  }
}