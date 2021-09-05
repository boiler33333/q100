use proconio::input;
use std::collections::VecDeque;
use std::i64::MAX;

fn bfs(
  n: usize,
  graph: &Vec<Vec<(usize, i64)>>,
  is_start: &Vec<bool>,
) -> Vec<i64> {
  let mut dist = vec![-1;  n];
  let mut que: VecDeque<usize> = VecDeque::new();
  for i in 0..n {
    if is_start[i] {
      dist[i] = 0;
      que.push_back(i);
    }
  }
  while let Some(from) = que.pop_front() {
    for &(to, _) in &graph[from] {
      if dist[to] == -1 {
        dist[to] = dist[from] + 1;
        que.push_back(to);
      }
    }
  }
  dist
}

fn dijkstra(
  n: usize,
  graph: &Vec<Vec<(usize, i64)>>,
  start: usize,
  goal: usize,
  is_zombie_city: &Vec<bool>,
) -> i64 {
  let mut dist = vec![MAX; n];
  let mut que: VecDeque<usize> = VecDeque::new();
  que.push_back(start);
  dist[start] = 0;
  while let Some(from) = que.pop_front() {
    for &(to, w) in &graph[from] {
      if is_zombie_city[to] {
        continue;
      }
      if dist[to] > dist[from] + w {
        dist[to] = dist[from] + w;
        que.push_back(to);
      }
    }
  }
  dist[goal]
}

fn main() {
  input! {
    n: usize, //町の数
    m: usize, //道路の数
    k: usize, //ゾンビに支配されている街の数
    s: i64,   //危険な町と判断するための道路の数
    p: i64,   //危険でない町での宿泊費
    q: i64,   //危険な町での宿泊費
    c: [usize; k], //ゾンビに支配されている町
    ab: [(usize, usize); m], //道路
  }
  let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push((b-1, p));
    graph[b-1].push((a-1, p));
  }
  let mut is_zombie_city = vec![false; n];
  for i in 0..k {
    is_zombie_city[c[i]-1] = true;
  }
  let dist = bfs(n, &graph, &is_zombie_city);
  for from in 0..n {
    for (to, w) in &mut graph[from] {
      if 0 < dist[*to] && dist[*to] <= s {
        *w = q;
      }
      if *to == n - 1 {
        *w = 0;
      }
    }
  }
  let ans = dijkstra(n, &graph, 0, n-1, &is_zombie_city);
  println!("{}", ans);
}