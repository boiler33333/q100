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

fn bfs(n: usize, g: &Vec<Vec<usize>>) -> Vec<i64> {
  let mut dist = vec![-1;  n];
  let mut que: VecDeque<usize> = VecDeque::new();
  dist[0] = 0;
  que.push_back(0);
  while let Some(from) = que.pop_front() {
    for &to in &g[from] {
      if dist[to] == -1 {
        dist[to] = dist[from] + 1;
        que.push_back(to);
      }
    }
  }
  dist
}

fn main() {
  let n: usize = read();
  let mut g = vec![vec![]; n];
  for _ in 0..n {
    let u: usize = read();
    let k: usize = read();
    for _ in 0..k {
      let v: usize = read();
      g[u-1].push(v-1);
    }
  }
  let dist = bfs(n, &g);
  for (i, v) in dist.iter().enumerate() {
    println!("{} {}", i+1, v);
  }
}