// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
#![allow(non_snake_case, unused_imports, unused_must_use)]
use std::io::{self, prelude::*};

fn main() {
    let (stdin, stdout) = (io::read_to_string(io::stdin()).unwrap(), io::stdout());
    let (mut stdin, mut buffer) = (stdin.split_whitespace(), io::BufWriter::new(stdout.lock()));

    macro_rules! input {
        ($t: tt, $n: expr) => {
            (0..$n).map(|_| input!($t)).collect::<Vec<_>>()
        };
        ($t: ty) => {
            stdin.next().unwrap().parse::<$t>().unwrap()
        };
    }

    const L: usize = 700;

    let N = input!(usize);
    let Q = input!(usize);

    let A = input!(u32, N);

    let B = (0..N / L)
        .map(|i| (i * L..i * L + L).map(|j| A[j]).min().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..Q {
        let mut l = input!(usize);
        let r = input!(usize);

        let mut ans = u32::MAX;

        while l < r {
            if l % L == 0 && l + L <= r {
                ans = std::cmp::min(ans, B[l / L]);
                l += L;
            } else {
                ans = std::cmp::min(ans, A[l]);
                l += 1;
            }
        }

        writeln!(buffer, "{}", ans);
    }
}
