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
    let wt = WaveletTree::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = wt.quantile(l, r, 0);

        writeln!(buffer, "{}", ans);
    }
}

struct WaveletTree {
    tree: Vec<Node>,
}

impl WaveletTree {
    fn new(array: &[u32]) -> Self {
        let mut tree = vec![TmpNode {
            left: None,
            right: None,
            bv: vec![],
        }];

        for &a in array {
            let mut now = 0;

            for i in (1..=32).rev() {
                let b = (a >> (i - 1)) & 1 == 1;

                tree[now].bv.push(b);

                if b {
                    match tree[now].right {
                        Some(index) => {
                            now = index as usize;
                        }
                        None => {
                            tree[now].right = Some(tree.len() as u32);
                            now = tree.len();
                            tree.push(TmpNode {
                                left: None,
                                right: None,
                                bv: vec![],
                            });
                        }
                    }
                } else {
                    match tree[now].left {
                        Some(index) => {
                            now = index as usize;
                        }
                        None => {
                            tree[now].left = Some(tree.len() as u32);
                            now = tree.len();
                            tree.push(TmpNode {
                                left: None,
                                right: None,
                                bv: vec![],
                            });
                        }
                    }
                }
            }
        }

        Self {
            tree: tree
                .into_iter()
                .map(|n| Node {
                    left: n.left,
                    right: n.right,
                    bv: BitVector::new(&n.bv),
                })
                .collect(),
        }
    }

    fn quantile(&self, mut left: usize, mut right: usize, mut k: usize) -> u32 {
        let mut now = 0;
        let mut ans = 0;

        for i in (1..=32).rev() {
            let zero_count = self.tree[now].bv.rank0(right) - self.tree[now].bv.rank0(left);

            if zero_count > k as u32 {
                left = self.tree[now].bv.rank0(left) as usize;
                right = self.tree[now].bv.rank0(right) as usize;
                now = self.tree[now].left.unwrap() as usize;
            } else {
                k -= zero_count as usize;
                ans |= 1 << (i - 1);

                left = self.tree[now].bv.rank1(left) as usize;
                right = self.tree[now].bv.rank1(right) as usize;
                now = self.tree[now].right.unwrap() as usize;
            }
        }

        ans
    }
}

struct TmpNode {
    left: Option<u32>,
    right: Option<u32>,
    bv: Vec<bool>,
}

struct Node {
    left: Option<u32>,
    right: Option<u32>,
    bv: BitVector,
}

struct BitVector {
    raw: Vec<u64>,
    cs_large: Vec<u32>,
    cs_small: Vec<u16>,
}

impl BitVector {
    const L: usize = 64 * 64;
    const S: usize = 64;

    fn new(vector: &[bool]) -> Self {
        let mut raw: Vec<u64> = vec![];

        for chunk in vector.chunks(Self::S) {
            let mut w = 0;

            for (b_idx, &b) in chunk.iter().enumerate() {
                if b {
                    w |= 1 << b_idx;
                }
            }

            raw.push(w);
        }

        let mut cs_large = vec![];
        let mut cs_small = vec![];

        let mut total = 0;
        let mut large_total = 0;

        for (i, &w) in raw.iter().enumerate() {
            let bit_idx = i * Self::S;

            if bit_idx % Self::L == 0 {
                cs_large.push(total);
                large_total = 0;
            }

            cs_small.push(large_total as u16);

            total += w.count_ones();
            large_total += w.count_ones();
        }

        if (raw.len() * Self::S) % Self::L == 0 {
            cs_large.push(total);
            large_total = 0;
        }

        cs_small.push(large_total as u16);

        Self {
            raw,
            cs_large,
            cs_small,
        }
    }

    fn rank1(&self, i: usize) -> u32 {
        if i == 0 {
            return 0;
        }

        self.cs_large[i / Self::L]
            + self.cs_small[i / Self::S] as u32
            + if i % Self::S != 0 {
                (self.raw[i / Self::S] & ((1 << (i % Self::S)) - 1)).count_ones()
            } else {
                0
            }
    }

    fn rank0(&self, i: usize) -> u32 {
        i as u32 - self.rank1(i)
    }
}
