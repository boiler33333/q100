use proconio::input;

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

fn main() {
  input! {
    n: usize,                 
    town: [(i64, i64); n],
  }
  let mut x = vec![(0, 0); n];
  let mut y = vec![(0, 0); n];
  for (i, &(xi, yi)) in town.iter().enumerate() {
    x[i] = (xi, i);
    y[i] = (yi, i);
  }
  x.sort();
  y.sort();
  let mut edge: Vec<(i64, usize, usize)> = Vec::new();
  for i in 0..n-1 {
    let w = x[i+1].0 - x[i].0;
    let s = x[i].1;
    let t = x[i+1].1;
    edge.push((w, s, t));
  }
  for i in 0..n-1 {
    let w = y[i+1].0 - y[i].0;
    let s = y[i].1;
    let t = y[i+1].1;
    edge.push((w, s, t));
  }
  edge.sort();
  let mut ans = 0;
  let mut uf = UnionFind::new(n);
  for (w, s, t) in edge {
    if uf.is_same(s, t) {
      continue;
    }
    ans += w;
    uf.unite(s, t);
  }
  println!("{}", ans);
}