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
    let queries = (0..Q)
        .map(|_| (input!(usize), input!(usize)))
        .collect::<Vec<_>>();

    let ans = offline_rmq(&A, &queries);

    for a in ans {
        writeln!(buffer, "{}", a);
    }
}

fn offline_rmq(array: &[u32], queries: &[(usize, usize)]) -> Vec<u32> {
    let n = array.len();
    let q = queries.len();

    let mut queries = queries
        .iter()
        .enumerate()
        .map(|(i, &(l, r))| (i, l, r - 1))
        .collect::<Vec<_>>();
    queries.sort_by_key(|t| t.2);

    let mut uf = UnionFind::new(n);
    let mut stack = vec![];
    let mut ans = vec![0; q];
    let mut now = 0;

    for i in 0..n {
        while !stack.is_empty() && array[*stack.last().unwrap()] > array[i] {
            let j = stack.pop().unwrap();
            uf.unite(j, i);
        }

        stack.push(i);

        while now < q && queries[now].2 == i {
            let (idx, left, _) = queries[now];
            let a = uf.find(left);
            ans[idx] = array[a];
            now += 1;
        }
    }

    ans
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).map(|i| i).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        root
    }

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.find(a);
        b = self.find(b);

        if a != b {
            self.parent[a] = b;
        }
    }
}
