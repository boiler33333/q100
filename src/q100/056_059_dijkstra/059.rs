use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  n: usize,
  graph: &Vec<Vec<usize>>,
  start: usize,
) -> Vec<i64> {
  let mut dist = vec![-1;  n];
  let mut que: VecDeque<usize> = VecDeque::new();
  dist[start] = 0;
  que.push_back(start);
  while let Some(from) = que.pop_front() {
    for &to in &graph[from] {
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
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize,
  goal: usize,
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
  input! {
    n: usize, //町の数
    k: usize, //道路の数
    cr: [(usize, i64); n], //運賃、タクシーで移動できる道路数
    ab: [(usize, usize); k], //道路
  }
  let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push(b-1);
    graph[b-1].push(a-1);
  }
  //タクシーで行ける街まで料金c円の道があるとする
  let mut graph2: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
  for i in 0..n {
    let dist = bfs(n, &graph, i);
    let (c, r) = cr[i];
    for j in 0..n {
      if i == j {
        continue;
      }
      if dist[j] > r {
        continue;
      }
      graph2[i].push((j, c))
    }
  }
  let ans = dijkstra(n, &graph2, 0, n-1);
  println!("{}", ans);
}