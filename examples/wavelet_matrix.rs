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
    let wm = WaveletMatrix::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = wm.quantile(l, r, 0);

        writeln!(buffer, "{}", ans);
    }
}

struct WaveletMatrix {
    size: usize,
    matrix: Vec<BitVector>,
}

impl WaveletMatrix {
    fn new(array: &[u32]) -> Self {
        let mut array_0: Vec<u32> = array.to_vec();
        let mut array_1: Vec<u32> = vec![];

        let mut matrix = vec![];

        for i in (1..=32).rev() {
            let mut array_0_nxt = vec![];
            let mut array_1_nxt = vec![];

            let mut vector = vec![];

            for a in array_0 {
                if (a >> (i - 1)) & 1 == 1 {
                    array_1_nxt.push(a);
                    vector.push(true);
                } else {
                    array_0_nxt.push(a);
                    vector.push(false);
                }
            }

            for a in array_1 {
                if (a >> (i - 1)) & 1 == 1 {
                    array_1_nxt.push(a);
                    vector.push(true);
                } else {
                    array_0_nxt.push(a);
                    vector.push(false);
                }
            }

            matrix.push(BitVector::new(&vector));

            array_0 = array_0_nxt;
            array_1 = array_1_nxt;
        }

        Self {
            size: array.len(),
            matrix,
        }
    }

    fn quantile(&self, mut left: usize, mut right: usize, mut k: usize) -> u32 {
        let mut ans = 0;

        for i in (1..=32).rev() {
            let l0 = self.matrix[32 - i].rank0(left);
            let r0 = self.matrix[32 - i].rank0(right);

            if k as u32 + l0 < r0 {
                left = l0 as usize;
                right = r0 as usize;
            } else {
                ans |= 1 << (i - 1);
                k -= r0 as usize - l0 as usize;
                let zeros = self.matrix[32 - i].rank0(self.size);
                left += (zeros - l0) as usize;
                right += (zeros - r0) as usize;
            }
        }

        ans
    }
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
