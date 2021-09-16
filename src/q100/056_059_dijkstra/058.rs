use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: &Vec<usize>,
) -> Vec<usize> {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  for &u in start {
    que.push_back(u);
    dist[u] = 0;
  }
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
  is_zombie_city: &Vec<bool>,
) -> usize {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  que.push_back(start);
  dist[start] = 0;
  while let Some(u) = que.pop_front() {
    for &(v, d) in &graph[u] {
      if is_zombie_city[v] {
        continue;
      }
      if dist[v] > dist[u] + d {
        dist[v] = dist[u] + d;
        que.push_back(v);
      }
    }
  }
  dist[goal]
}

fn main() {
  input! {
    n: usize, //町の数
    m: usize, //道路の本数
    k: usize, //ゾンビに支配されている町の数
    s: usize, //危険な町と判断される道路の本数
    p: usize, //宿泊費
    q: usize, //危険な町での宿泊費
    c: [usize; k],
    ab: [(usize, usize); m],
  }
  let mut zombie_town = vec![];
  for i in 0..k {
    zombie_town.push(c[i]-1);
  }
  let mut graph = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push((b-1, 0));
    graph[b-1].push((a-1, 0));
  }
  let dist = bfs(n, &graph, &zombie_town);
  for a in 0..n {
    for (b, c) in &mut graph[a] {
      if *b == n-1 {
        continue;
      }
      *c = if dist[*b] > s { p } else { q };
    }
  }
  let mut is_zombie_town = vec![false; n];
  for i in zombie_town {
    is_zombie_town[i] = true;
  }
  let ans = dijkstra(n, &graph, 0, n-1, &is_zombie_town);
  println!("{}", ans);
}