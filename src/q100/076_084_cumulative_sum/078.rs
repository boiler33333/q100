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
    h: usize, w: usize,
    k: usize,
    s: [String; h],
    abcd: [(usize, usize, usize, usize); k],
  }
  let mut jungle = vec![vec![0; w+1]; h+1];
  let mut ocean = vec![vec![0; w+1]; h+1];
  let mut ice = vec![vec![0; w+1]; h+1];
  for y in 0..h {
    for (x, c) in s[y].chars().enumerate() {
      match c {
        'J' => jungle[y+1][x+1] = 1,
        'O' => ocean[y+1][x+1] = 1,
        'I' => ice[y+1][x+1] = 1,
        _ => {},
      }
    }
  } 
  cumulative_sum(h+1, w+1, &mut jungle);
  cumulative_sum(h+1, w+1, &mut ocean);
  cumulative_sum(h+1, w+1, &mut ice);
  for (a, b, c, d) in abcd {
    print!("{} ", count(a, b, c, d, &jungle));
    print!("{} ", count(a, b, c, d, &ocean));
    println!("{}", count(a, b, c, d, &ice));
  }
}