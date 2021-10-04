use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
) -> Vec<usize> {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  dist[start] = 0;
  que.push_back(start);
  while let Some(u) = que.pop_front() {
    for &(v, _) in &graph[u] {
      if dist[v] == MAX {
        dist[v] = dist[u] + 1;
        que.push_back(v);
      }
    }
  }
  dist
}

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
  input! {
    n: usize, //町数
    k: usize, //道路数
    cr: [(usize, usize); n], //タクシーの運賃、連続して通れる最大道路数
    ab: [(usize, usize); k],
  }
  let mut graph = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push((b-1, 0));
    graph[b-1].push((a-1, 0));
  }
  //タクシーで行ける町まで料金c円の道があるとする
  let mut graph2 = vec![vec![]; n];
  for u in 0..n {
    let (c, r) = cr[u];
    let dist = bfs(n, &graph, u);
    for v in 0..n {
      if u == v {
        continue;
      }
      if dist[v] <= r {
        graph2[u].push((v, c));
      }
    }
  }
  let ans = dijkstra(n, &graph2, 0, n-1);
  println!("{}", ans);
}