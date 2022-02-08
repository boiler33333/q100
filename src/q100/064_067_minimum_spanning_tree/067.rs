use proconio::input;

struct UnionFind {
  par: Vec<Option<usize>>, // parent
  siz: Vec<usize>,         // size
}

impl UnionFind {
  fn new(n: usize) -> Self {
    UnionFind{ par: vec![None; n+1], siz: vec![1; n+1] }
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
    xy: [(i64, i64); n],
  }
  let mut x = vec![];
  let mut y = vec![];
  for i in 0..n {
    let (xi, yi) = xy[i];
    x.push((xi, i));
    y.push((yi, i));
  }
  x.sort();
  y.sort();
  let mut edges = vec![];
  for i in 1..n {
    let a = x[i-1].1;
    let b = x[i].1;
    let c = x[i].0 - x[i-1].0;
    edges.push((a, b, c));
  }
  for i in 1..n {
    let a = y[i-1].1;
    let b = y[i].1;
    let c = y[i].0 - y[i-1].0;
    edges.push((a, b, c));
  }
  edges.sort_by(|a, b| a.2.cmp(&b.2));
  let mut uf = UnionFind::new(n);
  let mut ans = 0;
  for (a, b, c) in edges {
    if uf.is_same(a, b) {
      continue;
    }
    uf.unite(a, b);
    ans += c;
  }
  println!("{}", ans);
}