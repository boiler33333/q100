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
  start: usize,
) -> Vec<usize> {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  dist[start] = 0;
  que.push_back(start);
  while let Some(u) = que.pop_front() {
    for &(v, d) in &graph[u] {
      if dist[v] > dist[u] + d  {
        dist[v] = dist[u] + d;
        que.push_back(v);
      }
    }
  }
  dist
}

fn main() {
  let v: usize = read();
  let e: usize = read();
  let r: usize = read();
  let mut graph = vec![vec![]; v];
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let d: usize = read();
    graph[s].push((t, d));
  }
  let ans = dijkstra(v, &graph, r);
  for i in 0..v {
    if ans[i] == MAX {
      println!("INF");
    } else {
      println!("{}", ans[i]);
    }
  }
}