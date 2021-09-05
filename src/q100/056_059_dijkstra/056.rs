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

fn dijkstra(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut dist = vec![MAX; n];
  let mut que: VecDeque<usize> = VecDeque::new();
  que.push_back(start);
  dist[start] = 0;
  while let Some(from) = que.pop_front() {
    for &(to, w) in &graph[from] {
      if dist[to] > dist[from] + w {
        dist[to] = dist[from] + w;
        que.push_back(to);
      }
    }
  }
  dist
}

fn main() {
  let v: usize = read();
  let e: usize = read();
  let r: usize = read();
  let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; v];
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let d: usize = read();
    g[s].push((t, d));
  }
  let dist = dijkstra(v, &g, r);
  for ans in dist {
    if ans < MAX {
      println!("{}", ans);
    } else {
      println!("INF");
    }
  }
}