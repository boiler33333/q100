use proconio::input;

fn dfs(
  graph: &Vec<Vec<usize>>,
  seen: &mut Vec<bool>,
  from: usize,
  score: &mut Vec<usize>,
) {
  seen[from] = true;
  for &to in &graph[from] {
    if !seen[to] {
      score[to] += score[from];
      dfs(graph, seen, to, score);
    }
  }
}

fn main() {
  input! {
    n: usize,
    q: usize,
    ab: [(usize, usize); n-1],
    px: [(usize, usize); q],
  }
  let mut graph = vec![vec![]; n];
  for &(a, b) in &ab {
    graph[a-1].push(b-1);
    graph[b-1].push(a-1);
  }
  let mut score = vec![0; n];
  for &(p, x) in &px {
    score[p-1] += x;
  }
  let mut seen = vec![false; n];
  dfs(&graph, &mut seen, 0, &mut score);
  let score: Vec<String> = score.iter().map(|x| x.to_string()).collect();
  let ans = score.join(" ");
  println!("{}", ans);
}