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
    n: usize, //都市の数
    m: usize, //道路の数
    mut k: usize, //本線を開催する年の個数
    mut abc: [(usize, usize, usize); m], //都市aとbを結ぶ料金cの道路
  }
  abc.sort_by(|x, y| x.2.cmp(&y.2));
  let mut ans = 0;
  let mut uf = UnionFind::new(n);
  for (a, b, c) in abc {
    if k >= n {
      break;
    }
    if uf.is_same(a-1, b-1) {
      continue;
    }
    ans += c;
    uf.unite(a-1, b-1);
    k += 1;
  }
  println!("{}", ans);
}