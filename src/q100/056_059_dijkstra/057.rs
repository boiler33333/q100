use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn dijkstra(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
  goal: usize
) -> usize {
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
  dist[goal]
}

fn main() {
  input! { n: usize, k: usize }
  let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
  for _ in 0..k {
    input! { x: usize }
    if x == 0 {
      input! { a: usize, b: usize }
      let ans = dijkstra(n, &g, a-1, b-1);
      if ans < MAX {
        println!("{}", ans);
      } else {
        println!("-1");
      }
    } else {
      input! { c: usize, d: usize, e: usize }
      g[c-1].push((d-1, e));
      g[d-1].push((c-1, e));
    }
  }
}