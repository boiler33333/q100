use proconio::input;

fn base8to10(n: u128) -> u128 {
    let mut res = 0;
    let mut n = n;
    let mut i = 1;
    while n > 0 {
        res += (n % 10) * i;
        i *= 8;
        n /= 10;
    }
    res
}

fn base10to9(n: u128) -> u128 {
    let mut res = 0;
    let mut n = n;
    let mut i = 1;
    while n > 0 {
        let mut v = n % 9;
        if v == 8 {
            v = 5;
        }
        res += v * i;
        i *= 10;
        n /= 9;
    }
    res
}

fn main() {
    input! {
        n: u128,
        k: usize,
    }
    let mut x = n;
    for _ in 0..k {
        x = base10to9(base8to10(x));
    }
    println!("{}", x);
}