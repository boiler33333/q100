use proconio::input;
use std::cmp::max;

fn cumulative_sum(h: usize, w: usize, graph: &mut Vec<Vec<usize>>) {
  for y in 0..h {
    for x in 1..w {
      graph[y][x] += graph[y][x-1];
    } 
  }
  for x in 0..w {
    for y in 1..h {
      graph[y][x] += graph[y-1][x];
    } 
  }
}

fn count(y1: usize, x1: usize, y2: usize, x2: usize, graph: &Vec<Vec<usize>>) -> usize {
  graph[y2][x2] + graph[y1-1][x1-1] - graph[y2][x1-1] - graph[y1-1][x2]
}

fn main() {
  input! {
    h: usize, w: usize, k: usize, v: usize,
    mut a: [[usize; w]; h],
  }
  a.insert(0, vec![0; w]);
  for y in 0..h+1 {
    a[y].insert(0, 0);
  }
  cumulative_sum(h+1, w+1, &mut a);
  let mut ans = 0;
  for y2 in 1..h+1 {
    for x2 in 1..w+1 {
      for y1 in 1..=y2 {
        for x1 in 1..=x2 {
          let s = (y2-y1+1) * (x2-x1+1);
          let cost = count(y1, x1, y2, x2, &a) + s * k;
          if cost <= v {
            ans = max(ans, s);
          }
        }
      }
    }
  }
  println!("{}", ans);
}