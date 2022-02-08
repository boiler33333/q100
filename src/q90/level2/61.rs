use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }
    let mut a = vec![];
    for (t, x) in tx {
        match t {
            1 => a.insert(0, x),
            2 => a.push(x),
            3 => println!("{}", a[x-1]),
            _ => unreachable!(),
        }
    }
}