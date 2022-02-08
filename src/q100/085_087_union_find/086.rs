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
  let mut ans = 0;
  for i in 0..m {
    let mut uf = UnionFind::new(n);
    for j in 0..m {
      if i == j {
        continue;
      }
      let (a, b) = ab[j];
      uf.unite(a, b);
    }
    if uf.size(1) < n {
      ans += 1;
    }
  }
  println!("{}", ans);
}