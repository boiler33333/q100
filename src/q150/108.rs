use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };
use std::cmp::max;
use std::i64::MIN;

struct RangeMinimumQuery {
  size: usize,
  max_value: Vec<i64>,
}

impl RangeMinimumQuery {
  fn new(_size: usize) -> Self {
    let mut size = 1;
    while size < _size {
      size <<= 1;
    }
    RangeMinimumQuery{
      size,
      max_value: vec![MIN/2; 2*size],
    }
  }
  fn update(&mut self, i: usize, x: i64) {
    let mut i = i + self.size - 1;
    self.max_value[i] = x;
    while i > 0 {
      i = (i-1) / 2;
      self.max_value[i] = max(self.max_value[2*i+1], self.max_value[2*i+2]);
    }
  }
  fn query(&self, mut a: usize, mut b: usize) -> i64 {
    let mut left = MIN/2;
    let mut right = MIN/2;
    a += self.size - 1;
    b += self.size - 1;
    while a < b {
        if (a & 1) == 0 {
            left = max(left, self.max_value[a]);
        }
        if (b & 1) == 0 {
            right = max(self.max_value[b - 1], right);
        }
        a = a / 2;
        b = (b - 1) / 2;
    }
    max(left, right)
  }
}

trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less => left = mid + 1,
        Equal | Greater => right = mid,
      }
    }
    left
  }

  fn upper_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less | Equal => left = mid + 1,
        Greater => right = mid,
      }
    }
    left
  }
}

fn setup(
  ret: &mut Vec<(i64, i64)>,
  xy: &[(i64, i64)],
  x: i64,
  y: i64,
){
  if xy.len() == 0 {
    ret.push((x, y));
  } else {
    setup(ret, &xy[1..], x, y); 
    setup(ret, &xy[1..], x - xy[0].0, y - xy[0].1); 
    setup(ret, &xy[1..], x + xy[0].0, y + xy[0].1); 
  }
}

fn main() {
  input! {
    n: usize, d: i64,
    xy: [(i64, i64); n],
  }
  let mut xy1 = vec![];
  let mut xy2 = vec![];
  setup(&mut xy1, &xy[0..n/2], 0, 0);
  setup(&mut xy2, &xy[n/2..n], 0, 0);
  xy2.sort();

  let m = xy2.len();
  let mut st = RangeMinimumQuery::new(m);
  let mut x2s = vec![0; m];
  for i in 0..m {
    x2s[i] = xy2[i].0;
    st.update(i, xy2[i].1);
  }

  let mut ans = 0;
  for &(x1, y1) in &xy1 {
    let s = x2s.lower_bound(&(-x1 - d));
    let t = x2s.upper_bound(&(-x1 + d));
    let y2 = st.query(s, t);
    ans = max(ans, y1 + y2);
  }
  println!("{}", ans);
}