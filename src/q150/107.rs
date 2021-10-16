use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    ab: [(usize, usize); m],
  }
  let mut ab1 = vec![];
  let mut ab2 = vec![];
  let mut ab3 = vec![];
  for &(a, b) in &ab {
    let a = a - 1;
    let b = b - 1;
    if b < n/2 {
      ab1.push((a, b));
    } else if n/2 <= a {
      ab2.push((a, b));
    } else {
      ab3.push((a, b));
    }
  }
  let x1 = setup(0, n/2, &ab1);
  let x2 = setup(n/2, n, &ab2);
  let mut ok = 0;
  let mut ng = n+1;
  while ng - ok > 1 {
    let mid = (ok + ng) / 2;
    if is_ok(&ab3, &x1, &x2, mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}

fn setup(
  l: usize,
  r: usize,
  ab: &[(usize, usize)],
) -> Vec<Vec<usize>> {
  let n = r - l;
  let mut ret = vec![vec![]; n+1];
  for state in 0..1<<n {
    let state = state << l;
    if !check(state, &ab) {
      continue;
    }
    let mut cnt = 0;
    for i in l..r {
      if state & 1 << i > 0 {
        cnt += 1;
      }
    }
    ret[cnt].push(state);
  }
  ret
}

fn is_ok(
  ab3: &[(usize, usize)],
  x1: &Vec<Vec<usize>>,
  x2: &Vec<Vec<usize>>,
  mid: usize,
) -> bool {
  for i in 0..=mid {
    let j = mid - i;
    if i >= x1.len() || j >= x2.len() {
      continue;
    }
    for &state1 in &x1[i] {
      for &state2 in &x2[j] {
        if check(state1|state2, &ab3) {
          return true;
        }
      }
    }
  }
  false
}

fn check(state: usize, ab: &[(usize, usize)]) -> bool {
  for &(a, b) in ab {
    if state & 1<<a > 0 && state & 1<<b > 0 {
      return false;
    }
  }
  true
}