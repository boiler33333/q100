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

fn main() {
  loop {
    let h: usize = read();
    if h == 0 {
      break;
    }
    let w: usize = 5;
    let k: usize = 3;
    let mut table = vec![vec![0; w]; h];
    for y in 0..h {
      for x in 0..w {
        table[y][x] = read();
      }
    }
    let point = play(h, w, k, &mut table);
    println!("{}", point);
  }
}

fn play(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> usize {
  let mut point = 0;
  loop {
    let blocks = remove_blocks(h, w, k, table);
    if blocks.len() == 0 {
      break;
    }
    for (v, d) in blocks {
      point += v * d;
    }
    down_blocks(h, w, table);
  }
  point
}

fn remove_blocks(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
  let mut res: Vec<(usize, usize)> = Vec::new();
  for y in 0..h {
    for x in 0..w {
      if table[y][x] == 0 {
        continue;
      }
      let mut d = 1;
      while x + d < w && table[y][x] == table[y][x+d] {
        d += 1;
      }
      if d < k {
        continue;
      }
      res.push((table[y][x], d));
      for j in 0..d {
        table[y][x+j] = 0;
      }
    }
  }
  res
}

fn down_blocks(h: usize, w: usize, table: &mut Vec<Vec<usize>>) {
  for hi in 1..h {
    for y in (hi..h).rev() {
      for x in 0..w {
        if table[y][x] == 0 {
          let tmp = table[y-1][x];
          table[y-1][x] = table[y][x];
          table[y][x] = tmp;
        }
      }
    }
  }
}