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
    let sqrt_tree: SqrtTree<Min<u32>> = SqrtTree::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = sqrt_tree.fold(l, r);

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

struct SqrtTree<S: SemiGroup> {
    size: usize,
    lg: usize,
    layer_lg: Vec<usize>,
    on_layer: Vec<usize>,
    array: Vec<S::S>,
    prefix: Vec<S::S>,
    suffix: Vec<S::S>,
    between: Vec<S::S>,
}

impl<S: SemiGroup> SqrtTree<S> {
    fn new(array: &[S::S]) -> Self {
        let size = array.len();
        let lg = size.next_power_of_two().trailing_zeros() as usize;

        let array = array.to_vec();
        let mut on_layer = vec![0; lg + 1];
        let mut layer_lg = vec![];
        let mut n_layer = 0;

        let mut i = lg;

        while i > 1 {
            on_layer[i] = n_layer;
            n_layer += 1;

            layer_lg.push(i);
            i = (i + 1) / 2;
        }

        for i in (0..lg).rev() {
            on_layer[i] = std::cmp::max(on_layer[i], on_layer[i + 1]);
        }

        let mut prefix = vec![array[0]; size * n_layer];
        let mut suffix = vec![array[0]; size * n_layer];

        let flatten_ps = |y, x| y * size + x;

        let mut between = vec![array[0]; (1 << lg) * n_layer];

        let flatten_bet = |y, x| y * (1 << lg) + x;

        for layer in 0..n_layer {
            let prev_b_sz = 1 << layer_lg[layer];
            let b_sz = 1 << ((layer_lg[layer] + 1) / 2);
            let b_cnt = 1 << (layer_lg[layer] / 2);

            for l in (0..size).step_by(prev_b_sz) {
                let r = std::cmp::min(l + prev_b_sz, size);

                for a in (l..r).step_by(b_sz) {
                    let b = std::cmp::min(a + b_sz, r);
                    prefix[flatten_ps(layer, a)] = array[a];

                    for i in a + 1..b {
                        prefix[flatten_ps(layer, i)] =
                            S::op(prefix[flatten_ps(layer, i - 1)], array[i]);
                    }

                    suffix[flatten_ps(layer, b - 1)] = array[b - 1];

                    for i in (a..b - 1).rev() {
                        suffix[flatten_ps(layer, i)] =
                            S::op(array[i], suffix[flatten_ps(layer, i + 1)]);
                    }
                }

                for i in 0..b_cnt {
                    if l + i * b_sz >= r {
                        break;
                    }
                    let mut val = suffix[flatten_ps(layer, l + i * b_sz)];
                    between[flatten_bet(layer, l + i * b_cnt + i)] = val;

                    for j in i + 1..b_cnt {
                        if l + j * b_sz >= r {
                            break;
                        }
                        val = S::op(val, suffix[flatten_ps(layer, l + j * b_sz)]);
                        between[flatten_bet(layer, l + i * b_cnt + j)] = val;
                    }
                }
            }
        }

        Self {
            size,
            lg,
            layer_lg,
            on_layer,
            array,
            prefix,
            suffix,
            between,
        }
    }

    fn fold(&self, left: usize, mut right: usize) -> S::S {
        right -= 1;

        if left == right {
            return self.array[left];
        }
        if left + 1 == right {
            return S::op(self.array[left], self.array[right]);
        }

        let layer = self.on_layer[(64 - (left ^ right).leading_zeros()) as usize];
        let b_sz = 1 << ((self.layer_lg[layer] + 1) / 2);
        let b_cnt = 1 << (self.layer_lg[layer] / 2);
        let a = (left >> self.layer_lg[layer]) << self.layer_lg[layer];
        let left_block = (left - a) / b_sz + 1;
        let right_block = (right - a) / b_sz - 1;

        let flatten_ps = |y, x| y * self.size + x;
        let flatten_mid = |y, x| y * (1 << self.lg) + x;

        let mut val = self.suffix[flatten_ps(layer, left)];

        if left_block <= right_block {
            val = S::op(
                val,
                self.between[flatten_mid(layer, a + left_block * b_cnt + right_block)],
            );
        }

        val = S::op(val, self.prefix[flatten_ps(layer, right)]);

        val
    }
}
