use proconio::input;

fn cumulative_sum(h: usize, w: usize, graph: &mut Vec<Vec<usize>>) {
  for y in 0..h {
    for x in 1..w {
      graph[y][x] += graph[y][x-1];
    } 
  }
  for x in 0..w {
    for y in 1..h {
      graph[y][x] += graph[y-1][x];
    } 
  }
}

fn count(y1: usize, x1: usize, y2: usize, x2: usize, graph: &Vec<Vec<usize>>) -> usize {
  graph[y2][x2] + graph[y1-1][x1-1] - graph[y2][x1-1] - graph[y1-1][x2]
}

fn main() {
  input! {
    n: usize,  //都市の数
    m: usize,  //列車の数
    o: usize,  //質問の数
    lr: [(usize, usize); m],  //列車の区間
    pq: [(usize, usize); o],  //質問の区間
  }
  let mut graph = vec![vec![0; n+1]; n+1];
  for (l, r) in lr {
    graph[l][r] += 1;
  }
  cumulative_sum(n+1, n+1, &mut graph);
  for (p, q) in pq {
    let ans = count(p, p, q, q, &graph);
    println!("{}", ans);
  }
}