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
    n: usize, m: usize,
    mut k: usize,
    mut abc: [(usize, usize, usize); m],
  }
  let ans = solve(n, k, &mut abc);
  println!("{}", ans);
}

fn solve(n: usize, k: usize, abc: &mut Vec<(usize, usize, usize)>) -> usize {
  abc.sort_by(|x, y| x.2.cmp(&y.2));
  let mut uf = UnionFind::new(n);
  let mut i = k;
  let mut res = 0;
  for &mut (a, b, c) in abc {
    if i >= n {
      break;
    }
    if uf.is_same(a-1, b-1) {
      continue;
    }
    uf.unite(a-1, b-1);
    i += 1;
    res += c;
  }
  res
}

#[test]
fn test_solve_1() {
  let n = 4;
  let k = 1;
  let mut abc = vec![(1,2,2),(2,3,9),(2,4,5)];
  let got = solve(n, k, &mut abc);
  assert_eq!(got, 16);
}

#[test]
fn test_solve_2() {
  let n = 5;
  let k = 2;
  let mut abc = vec![(1,2,5),(1,3,3),(2,3,4),(2,5,7),(3,4,6),(4,5,5)];
  let got = solve(n, k, &mut abc);
  assert_eq!(got, 12);
}