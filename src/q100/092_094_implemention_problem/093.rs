use proconio::input;
use std::cmp::max;

fn main() {
  input! { h: usize, w: usize, k: usize, c: [String; h] }
  let c: Vec<Vec<usize>> = c
    .iter()
    .map(|s| s.bytes().map(|b| (b-48) as usize).collect())
    .collect();
  let mut ans = 0;
  for y in 0..h {
    for x in 0..w {
      let mut table = c.clone();
      table[y][x] = 0;
      fall(h, w, &mut table);
      let points = solve(h, w, k, &mut table); 
      ans = max(ans, points);
    }
  }
  println!("{}", ans);
}

fn solve(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> usize {
  let mut points = 0;
  let mut times = 0;
  loop {
    let blocks = delete(h, w, k, table);
    if blocks.len() == 0 {
      break;
    }
    for (v, n) in blocks {
      points += 2usize.pow(times) * v * n;
    }
    fall(h, w, table);
    times += 1;
  }
  points
}

fn delete(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
  let mut blocks = vec![];
  for y in 0..h {
    for x in 0..w {
      if table[y][x] == 0 {
        continue;
      }
      let mut n = 1;
      while x + n < w && table[y][x] == table[y][x+n] {
        n += 1;
      }
      if n < k {
        continue;
      }
      blocks.push((table[y][x], n));
      for i in 0..n {
        table[y][x+i] = 0;
      }
    }
  }
  blocks
}

fn fall(h: usize, w: usize, table: &mut Vec<Vec<usize>>) {
  for i in 1..h {
    for y in (i..h).rev() {
      for x in 0..w {
        if table[y][x] == 0 {
          let tmp = table[y][x];
          table[y][x] = table[y-1][x];
          table[y-1][x] = tmp;
        }
      }
    }
  }
}