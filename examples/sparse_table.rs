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

    let sparse_table: SparseTable<Min<u32>> = SparseTable::build(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = sparse_table.fold(l..r);

        writeln!(buffer, "{}", ans);
    }
}

trait Monoid {
    type S: Copy;
    fn op(lhs: Self::S, rhs: Self::S) -> Self::S;
    const E: Self::S;
}

struct Min<S> {
    _marker: std::marker::PhantomData<S>,
}

impl Monoid for Min<u32> {
    type S = u32;
    fn op(lhs: Self::S, rhs: Self::S) -> Self::S {
        std::cmp::min(lhs, rhs)
    }
    const E: Self::S = u32::MAX;
}

struct SparseTable<M: Monoid> {
    size: usize,
    table: Vec<M::S>,
}

impl<M: Monoid> SparseTable<M> {
    fn build(array: &[M::S]) -> Self {
        let size = array.len();
        let height = (usize::BITS - size.leading_zeros()) as usize;

        let mut table = vec![M::E; size * height];

        for i in 0..size {
            table[i] = array[i];
        }

        let flatten = |y, x| y * size + x;

        for h in 1..height {
            for i in 0..size {
                if i + (1 << h) > size {
                    break;
                }

                table[flatten(h, i)] = M::op(
                    table[flatten(h - 1, i)],
                    table[flatten(h - 1, i + (1 << (h - 1)))],
                );
            }
        }

        Self { size, table }
    }

    fn _fold(&self, l: usize, r: usize) -> M::S {
        assert!(l < self.size && r <= self.size);

        if r == l + 1 {
            return self.table[l];
        }

        let length = r - l;
        let h = (usize::BITS - 1 - length.leading_zeros()) as usize;
        let w = 1 << h;

        let flatten = |y, x| y * self.size + x;

        M::op(self.table[flatten(h, l)], self.table[flatten(h, r - w)])
    }

    fn fold<R: std::ops::RangeBounds<usize>>(&self, range: R) -> M::S {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.size,
        };

        self._fold(left, right)
    }
}
