use std::io::*;
use std::str::FromStr;

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

fn main() {
  let v: usize = read();
  let e: usize = read();
  let mut wst: Vec<(usize, usize, usize)> = Vec::new();
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let w: usize = read();
    wst.push((w, s, t));
  }
  wst.sort();
  let mut ans = 0;
  let mut uf = UnionFind::new(v);
  for (w, s, t) in wst {
    if uf.is_same(s, t) {
      continue;
    }
    ans += w;
    uf.unite(s, t);
  }
  println!("{}", ans);
}