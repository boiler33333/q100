use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn dijkstra(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
  goal: usize,
) -> usize {
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
  dist[goal]
}

fn main() {
  input! { n: usize, k: usize }
  let mut graph = vec![vec![]; n];
  for _ in 0..k {
    input! { cmd: usize }
    if cmd == 0 {
      input! { a: usize, b: usize }
      let ans = dijkstra(n, &graph, a-1, b-1);
      if ans == MAX {
        println!("-1");
      } else {
        println!("{}", ans);
      }
    } else {
      input! { c: usize, d: usize, e: usize }
      graph[c-1].push((d-1, e));
      graph[d-1].push((c-1, e));
    }
  }
}