struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut vec = vec![];
        for i in 0..n {
            vec.push(i)
        }
        UnionFind { par: vec }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        return self.par[x];
    }

    fn unite(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);
        if (rx == ry) {
            return;
        };
        self.par[rx] = ry;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        return rx == ry;
    }

    fn count(&mut self) -> usize {
        let l = self.par.len();
        let mut sum = 0;
        for i in 0..l {
            if i == self.par[i] {
                sum += 1
            }
        }
        return sum;
    }
}
