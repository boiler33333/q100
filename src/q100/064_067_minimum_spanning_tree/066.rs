use std::io::*;
use std::str::FromStr;
use std::cmp::PartialOrd;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

struct UnionFind {
  par: Vec<Option<usize>>, //parent
  siz: Vec<usize>,         //size
}

impl UnionFind {
  fn new(n: usize) -> Self {
    UnionFind{ par: vec![None; n], siz: vec![1; n] }
  }

  fn root(&self, x: usize) -> usize {
    match self.par[x] {
      Some(v) => self.root(v),
      None => x,
    }
  }

  fn is_same(&self, x: usize, y: usize) -> bool {
    self.root(x) == self.root(y)
  }

  fn unite(&mut self, x: usize, y: usize) {
    let x = self.root(x);
    let y = self.root(y);
    if x == y {
      return;
    }
    if self.siz[x] < self.siz[y] {
      self.par[x] = Some(y);
      self.siz[y] += self.siz[x];
    } else {
      self.par[y] = Some(x);
      self.siz[x] += self.siz[y];
    }
  }
}

fn solve(n: usize, cell: &Vec<(f64, f64, f64, f64)>) {
  let mut edges: Vec<(f64, usize, usize)> = vec![];
  for from in 0..n {
    for to in from+1..n {
      let (x1, y1, z1, r1) = cell[from];
      let (x2, y2, z2, r2) = cell[to];
      let dx = x2 - x1;
      let dy = y2 - y1;
      let dz = z2 - z1;
      let dist = (dx*dx + dy*dy + dz*dz).sqrt();
      let dist = dist - (r1 + r2);
      if dist > 0.0 {
        edges.push((dist, from, to));
      } else {
        edges.push((0.0, from, to));
      }
    }
  }
  edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  let mut ans = 0.0;
  let mut uf = UnionFind::new(n);
  for (dist, from, to) in edges {
    if uf.is_same(from, to) {
      continue;
    }
    ans += dist;
    uf.unite(from, to);
  }
  println!("{:.3}", ans);
}

fn main() {
  loop {
    let n: usize = read(); //セルの数
    if n == 0 {
      break;
    }
    let mut cell: Vec<(f64, f64, f64, f64)> = Vec::new();
    for _ in 0..n {
      let x: f64 = read(); //球の中心のx座標
      let y: f64 = read(); //球の中心のy座標
      let z: f64 = read(); //球の中心のz座標
      let r: f64 = read(); //球の半径
      cell.push((x, y, z, r));
    }
    solve(n, &cell);
  }
}