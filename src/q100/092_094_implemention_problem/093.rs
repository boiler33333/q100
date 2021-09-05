use proconio::input;
use std::cmp::max;

fn main() {
  input! { h: usize, w: usize, k: usize, s: [String; h] }
  let mut c: Vec<Vec<usize>> = vec![vec![0; w]; h];
  for y in 0..h {
    for (x, v) in s[y].bytes().enumerate() {
      c[y][x] = (v - 48) as usize;
    }
  }
  let mut ans = 0;
  for y in 0..h {
    for x in 0..w {
      let mut c2 = c.clone();
      c2[y][x] = 0;
      let point = play(h, w, k, &mut c2);
      ans = max(ans, point);
    }
  }
  println!("{}", ans);
}

fn play(h: usize, w: usize, k: usize, c: &mut Vec<Vec<usize>>) -> usize {
  let mut point = 0;
  let mut times = 0;
  down_blocks(h, w, c);
  loop {
    let blocks = remove_blocks(h, w, k, c);
    if blocks.len() == 0 {
      break;
    }
    for (v, d) in blocks {
      point += 2usize.pow(times) * v * d;
    }
    down_blocks(h, w, c);
    times += 1;
  }
  point
}

fn remove_blocks(h: usize, w: usize, k: usize, c: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
  let mut res: Vec<(usize, usize)> = Vec::new();
  for y in 0..h {
    for x in 0..w {
      if c[y][x] == 0 {
        continue;
      }
      let mut d = 1;
      while x + d < w && c[y][x] == c[y][x+d] {
        d += 1;
      }
      if d < k {
        continue;
      }
      res.push((c[y][x], d));
      for j in 0..d {
        c[y][x+j] = 0;
      }
    }
  }
  res
}

fn down_blocks(h: usize, w: usize, c: &mut Vec<Vec<usize>>) {
  for hi in 1..h {
    for y in (hi..h).rev() {
      for x in 0..w {
        if c[y][x] == 0 {
          let tmp = c[y-1][x];
          c[y-1][x] = c[y][x];
          c[y][x] = tmp;
        }
      }
    }
  }
}