use std::cmp::PartialOrd;
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
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    let mut xyzr = vec![];
    for _ in 0..n {
      let x: f64 = read();
      let y: f64 = read();
      let z: f64 = read();
      let r: f64 = read();
      xyzr.push((x,y,z,r));
    }
    let ans = solve(n, &xyzr);
    println!("{:.3}", ans);
  }
}

fn solve(
  n: usize,
  xyzr: &[(f64,f64,f64,f64)],
) -> f64 {
  let mut edges = vec![];
  for a in 0..n {
    for b in a+1..n {
      let (x1, y1, z1, r1) = xyzr[a];
      let (x2, y2, z2, r2) = xyzr[b];
      let dx = x2 - x1;
      let dy = y2 - y1;
      let dz = z2 - z1;
      let c = (dx*dx + dy*dy + dz*dz).sqrt() - (r1+r2);
      if c > 0.0 {
        edges.push((a, b, c));
      } else {
        edges.push((a, b, 0.0));
      }
    }
  }
  edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
  let mut uf = UnionFind::new(n);
  let mut ret = 0.0;
  for (a, b, c) in edges {
    if uf.is_same(a, b) {
      continue;
    }
    uf.unite(a, b);
    ret += c;
  }
  ret
}