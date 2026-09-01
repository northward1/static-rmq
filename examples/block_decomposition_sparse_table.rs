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
    let st = BlockDecompositionSparseTable::new(&A);

    for _ in 0..Q {
        let l = input!(usize);
        let r = input!(usize);

        let ans = st.fold(l, r);

        writeln!(buffer, "{}", ans);
    }
}

struct BlockDecompositionSparseTable {
    array: Vec<u32>,
    min_sparse_table: MinSparseTable,
    dat: Vec<u32>,
    suffix: Vec<u32>,
    prefix: Vec<u32>,
}

impl BlockDecompositionSparseTable {
    const BLOCK_SIZE: usize = 16;

    fn new(array: &[u32]) -> Self {
        let size = array.len();
        let array = array.to_vec();

        let mut prefix = array.clone();
        let mut suffix = array.clone();

        for i in 1..size {
            if i % 16 != 0 {
                prefix[i] = std::cmp::min(prefix[i - 1], array[i]);
            }
        }

        for i in (1..size).rev() {
            if i % 16 != 0 {
                suffix[i - 1] = std::cmp::min(array[i - 1], suffix[i]);
            }
        }

        let min_sparse_table = MinSparseTable::build(
            &(0..size / Self::BLOCK_SIZE)
                .map(|i| suffix[i * Self::BLOCK_SIZE])
                .collect::<Vec<_>>(),
        );

        let mut dat = vec![0; size];

        let mut stack = 0u32;

        for i in (0..size).rev() {
            stack = (stack << 1) & 65535;

            while stack > 0 {
                let k = stack.trailing_zeros() as usize;

                if std::cmp::min(array[i], array[i + k]) != array[i] {
                    break;
                }

                stack &= !(1 << k);
            }

            stack |= 1;
            dat[i] = stack;
        }

        Self {
            array,
            min_sparse_table,
            dat,
            suffix,
            prefix,
        }
    }

    fn fold(&self, left: usize, mut right: usize) -> u32 {
        if left / 16 == (right - 1) / 16 {
            let d = self.dat[left] & ((1 << (right - left)) - 1);
            return self.array[left + 31 - d.leading_zeros() as usize];
        }

        right -= 1;

        let a = left / Self::BLOCK_SIZE;
        let b = right / Self::BLOCK_SIZE;

        let mut x = if a + 1 < b {
            self.min_sparse_table.fold(a + 1..b)
        } else {
            u32::MAX
        };
        x = std::cmp::min(x, self.suffix[left]);
        x = std::cmp::min(x, self.prefix[right]);

        x
    }
}

struct MinSparseTable {
    size: usize,
    table: Vec<u32>,
}

impl MinSparseTable {
    fn build(array: &[u32]) -> Self {
        let size = array.len();
        let height = (usize::BITS - size.leading_zeros()) as usize;

        let mut table = vec![0; size * height];

        for i in 0..size {
            table[i] = array[i];
        }

        let flatten = |y, x| y * size + x;

        for h in 1..height {
            for i in 0..size {
                if i + (1 << h) > size {
                    break;
                }

                table[flatten(h, i)] = std::cmp::min(
                    table[flatten(h - 1, i)],
                    table[flatten(h - 1, i + (1 << (h - 1)))],
                );
            }
        }

        Self { size, table }
    }

    fn _fold(&self, l: usize, r: usize) -> u32 {
        assert!(l < self.size && r <= self.size);

        if r == l + 1 {
            return self.table[l];
        }

        let length = r - l;
        let h = (usize::BITS - 1 - length.leading_zeros()) as usize;
        let w = 1 << h;

        let flatten = |y, x| y * self.size + x;

        std::cmp::min(self.table[flatten(h, l)], self.table[flatten(h, r - w)])
    }

    fn fold<R: std::ops::RangeBounds<usize>>(&self, range: R) -> u32 {
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
