use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::Included;

fn main() {
  input! {
    n: usize,
    m: usize,
    tlr: [(usize, usize, usize); m],
  }
  let mut set: BTreeSet<usize> = (1..=n).collect();
  let mut ans = 0;
  for &(t, l, r) in tlr.iter().rev() {
    let mut tmp = vec![];
    for &i in set.range((Included(&l), Included(&r))) {
      ans += t;
      tmp.push(i);
    }
    for i in tmp {
      set.remove(&i);
    }
  }
  println!("{}", ans);
}