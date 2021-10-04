use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  n: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  k: usize,
  start: &[usize],
) -> Vec<usize> {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  for i in 0..k {
    dist[start[i]] = 0;
    que.push_back(start[i]);
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
  is_zombie_town: &[bool],
) -> usize {
  let mut dist = vec![MAX; n];
  let mut que = VecDeque::new();
  dist[0] = 0;
  que.push_back(0);
  while let Some(u) = que.pop_front() {
    for &(v, d) in &graph[u] {
      if is_zombie_town[v] {
        continue;
      }
      if dist[v] > dist[u] + d  {
        dist[v] = dist[u] + d;
        que.push_back(v);
      }
    }
  }
  dist[n-1]
}

fn main() {
  input! {
    n: usize, //町数
    m: usize, //道路数
    k: usize, //ゾンビに支配されている町数
    s: usize, //危険な町と判定するための道路数
    p: usize, //危険でない町での宿泊費
    q: usize, //危険な町での宿泊費
    c: [usize; k],
    ab: [(usize, usize); m],
  }
  let c: Vec<usize> = c.iter().map(|x| x-1).collect();
  let mut graph = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push((b-1, p));
    graph[b-1].push((a-1, p));
  }
  let dist = bfs(n, &graph, k, &c);
  for u in 0..n {
    for (v, p) in &mut graph[u] {
      if dist[*v] <= s {
        *p = q;
      }
      if *v == n-1 {
        *p = 0;
      }
    }
  }
  let mut is_zombie_town = vec![false; n];
  for i in 0..k {
    is_zombie_town[c[i]] = true;
  }
  let ans = dijkstra(n, &graph, &is_zombie_town);
  println!("{}", ans);
}