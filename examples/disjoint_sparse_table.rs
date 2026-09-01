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

    let N = input!(usize);
    let Q = input!(usize);

    let A = input!(u32, N);
    let dst: DisjointSparseTable<Min<u32>> = DisjointSparseTable::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = dst.fold(l, r);

        writeln!(buffer, "{}", ans);
    }
}

trait SemiGroup {
    type S: Copy;
    fn op(lhs: Self::S, rhs: Self::S) -> Self::S;
}

struct Min<S> {
    _marker: std::marker::PhantomData<S>,
}

impl SemiGroup for Min<u32> {
    type S = u32;
    fn op(lhs: Self::S, rhs: Self::S) -> Self::S {
        std::cmp::min(lhs, rhs)
    }
}

struct DisjointSparseTable<S: SemiGroup> {
    array: Vec<S::S>,
    table: Vec<S::S>,
}

impl<S: SemiGroup> DisjointSparseTable<S> {
    fn new(array: &[S::S]) -> Self {
        let size = array.len();
        let K = size.next_power_of_two().trailing_zeros() as usize;

        let flatten = |y, x| y * size + x;

        let mut table = vec![];
        let array = array.to_vec();

        for k in 0..K {
            table.append(&mut array.clone());

            let w = 1 << k;

            for a in (0..size - w).step_by(2 * w) {
                let b = std::cmp::min(a + 2 * w, size);
                let c = a + w;

                for i in (a..c - 1).rev() {
                    table[flatten(k, i)] = S::op(array[i], table[flatten(k, i + 1)]);
                }

                for i in c + 1..b {
                    table[flatten(k, i)] = S::op(table[flatten(k, i - 1)], array[i]);
                }
            }
        }

        Self { array, table }
    }

    fn fold(&self, left: usize, right: usize) -> S::S {
        if left + 1 == right {
            return self.array[left];
        }

        let k = 63 - (left ^ (right - 1)).leading_zeros() as usize;
        let flatten = |y, x| y * self.array.len() + x;
        S::op(
            self.table[flatten(k, left)],
            self.table[flatten(k, right - 1)],
        )
    }
}
