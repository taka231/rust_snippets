use cargo_snippet::snippet;

use crate::monoid::{MapMonoid, Monoid};

#[snippet("lazy_segtree")]
#[snippet(include = "lazy_segtree_impl")]
#[snippet(include = "lazy_segtree_from_vec")]
#[snippet(include = "monoid")]
#[snippet(include = "monoid_util")]
#[snippet(include = "map_monoid")]
#[snippet(include = "map_monoid_util")]
#[derive(Debug)]
struct LazySegtree<F: MapMonoid> {
    n: usize,
    size: usize,
    log: usize,
    data: Vec<F::X>,
    lazy: Vec<F>,
}

#[snippet("lazy_segtree_impl")]
impl<F> LazySegtree<F>
where
    F: MapMonoid + Clone + PartialEq,
    F::X: Clone,
{
    pub fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        let log = (size as f64).log2().ceil() as usize;
        Self {
            n,
            size,
            log,
            data: vec![F::X::id(); 2 * n - 1],
            lazy: vec![F::id(); 2 * n - 1],
        }
    }
    fn update(&mut self, k: usize) {
        self.data[k] = self.data[2 * k + 1].op(&self.data[2 * k + 2]);
    }
    fn all_apply(&mut self, k: usize, f: F) {
        self.data[k] = f.mapping(&self.data[k]);
        if k < self.n {
            self.lazy[k] = self.lazy[k].op(&f);
        }
    }
    fn push(&mut self, k: usize) {
        if self.lazy[k] == F::id() {
            return;
        }
        self.all_apply(2 * k + 1, self.lazy[k].clone());
        self.all_apply(2 * k + 2, self.lazy[k].clone());
        self.lazy[k] = F::id();
    }
    pub fn set(&mut self, p: usize, x: F::X) {
        assert!(p < self.size);
        let p = p + self.n;
        for i in (1..=self.log).rev() {
            self.push((p >> i) - 1);
        }
        self.data[p - 1] = x;
        for i in 1..=self.log {
            self.update((p >> i) - 1);
        }
    }
    pub fn get(&mut self, p: usize) -> F::X {
        assert!(p < self.size);
        let p = p + self.n;
        for i in (1..=self.log).rev() {
            self.push((p >> i) - 1);
        }
        self.data[p - 1].clone()
    }
    pub fn prod<R>(&mut self, range: R) -> F::X
    where
        R: std::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.n,
        };
        if start >= end {
            return F::X::id();
        }
        self.prod_rec(0, start, end, 0, self.n)
    }
    fn prod_rec(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) -> F::X {
        if search_left <= left && right <= search_right {
            self.data[tree_index].clone()
        } else if right <= search_left || search_right <= left {
            F::X::id()
        } else {
            if self.lazy[tree_index] != F::id() {
                self.propagate(tree_index, left, right, left, right);
            }
            let mid = (left + right) / 2;
            let (left_t_index, right_t_index) = self.get_children_index(tree_index);
            let left_value = self.prod_rec(left_t_index, search_left, search_right, left, mid);
            let right_value = self.prod_rec(right_t_index, search_left, search_right, mid, right);
            left_value.op(&right_value)
        }
    }
    pub fn all_prod(&self) -> F::X {
        self.data[0].clone()
    }
    fn get_children_index(&self, tree_index: usize) -> (usize, usize) {
        (tree_index * 2 + 1, tree_index * 2 + 2)
    }
    pub fn apply(&mut self, p: usize, f: F) {
        assert!(p < self.size);
        let p = p + self.n - 1;
        for i in (1..=self.log).rev() {
            self.push((p >> i) - 1);
        }
        self.data[p] = f.mapping(&self.data[p]);
        for i in 1..=self.log {
            self.update((p >> i) - 1);
        }
    }
    pub fn apply_range<R>(&mut self, f: F, range: R)
    where
        R: std::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => self.n,
        };
        self.apply_rec(f, 0, start, end, 0, self.n);
    }
    fn apply_rec(
        &mut self,
        f: F,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        if right <= search_left || search_right <= left {
            return;
        }
        self.lazy[tree_index] = f.op(&self.lazy[tree_index]);
        if search_left <= left && right <= search_right {
            self.data[tree_index] = f.mapping(&self.data[tree_index]);
        } else {
            self.propagate(tree_index, search_left, search_right, left, right);
            let (left_t_index, right_t_index) = self.get_children_index(tree_index);
            self.data[tree_index] = self.data[left_t_index].op(&self.data[right_t_index]);
        }
    }
    fn propagate(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        let lazy = self.lazy[tree_index].clone();
        self.lazy[tree_index] = F::id();
        let mid = (left + right) / 2;
        let (left_t_index, right_t_index) = self.get_children_index(tree_index);
        self.apply_rec(
            lazy.clone(),
            left_t_index,
            search_left,
            search_right,
            left,
            mid,
        );
        self.apply_rec(lazy, right_t_index, search_left, search_right, mid, right);
    }
}

#[snippet("lazy_segtree_from_vec")]
impl<F> From<Vec<F::X>> for LazySegtree<F>
where
    F::X: Clone,
    F: MapMonoid + Clone + PartialEq,
{
    fn from(v: Vec<F::X>) -> Self {
        let size = v.len();
        let mut segtree = LazySegtree::new(size);
        segtree.data[segtree.n - 1..(segtree.n - 1 + size)].clone_from_slice(&v);
        for i in (0..segtree.n - 1).rev() {
            segtree.update(i);
        }
        segtree
    }
}
