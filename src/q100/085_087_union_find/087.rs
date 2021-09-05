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

  fn size(&self, x: usize) -> usize {
    self.siz[self.root(x)]
  }
}

fn main() {
  input! {
    n: usize, m: usize,
    ab: [(usize, usize); m],
  }
  let mut uf = UnionFind::new(n);
  let mut ans = vec![0; m];
  ans[m-1] = n * (n-1) / 2;
  for j in (1..m).rev() {
    let (a, b) = ab[j];
    if uf.is_same(a, b) {
      ans[j-1] = ans[j];
    } else {
      ans[j-1] = ans[j] - uf.size(a) * uf.size(b);
      uf.unite(a, b);
    }
  }
  for j in 0..m {
    println!("{}", ans[j]);
  }
}