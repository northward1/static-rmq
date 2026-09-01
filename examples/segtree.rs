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
    let stree: SegTree<Min<u32>> = SegTree::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = stree.fold(l..r);

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

struct SegTree<M: Monoid> {
    size: usize,
    tree: Vec<M::S>,
}

impl<M: Monoid> SegTree<M> {
    fn new(array: &[M::S]) -> Self {
        let size = array.len();
        let tree = {
            let mut tree = vec![M::E; size];
            tree.append(&mut array.to_vec());

            for i in (1..size).rev() {
                tree[i] = M::op(tree[i << 1], tree[i << 1 | 1]);
            }

            tree
        };

        return Self { size, tree };
    }

    fn insert(&mut self, mut i: usize, s: M::S) {
        i += self.size;

        self.tree[i] = s;

        while i > 1 {
            i >>= 1;
            self.tree[i] = M::op(self.tree[i << 1], self.tree[i << 1 | 1]);
        }
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

        return self._fold(left, right);
    }

    fn _fold(&self, mut left: usize, mut right: usize) -> M::S {
        left += self.size;
        right += self.size;
        let (mut sl, mut sr) = (M::E, M::E);

        while left < right {
            if left & 1 == 1 {
                sl = M::op(sl, self.tree[left]);
                left += 1;
            }

            if right & 1 == 1 {
                right ^= 1;
                sr = M::op(self.tree[right], sr);
            }

            left >>= 1;
            right >>= 1;
        }

        return M::op(sl, sr);
    }
}
