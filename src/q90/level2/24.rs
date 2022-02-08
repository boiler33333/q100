use proconio::input;

fn main() {
    input!  {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut d = 0;
    for i in 0..n {
        d += (a[i] - b[i]).abs();
    }
    if (d % 2 == k % 2) && d <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}