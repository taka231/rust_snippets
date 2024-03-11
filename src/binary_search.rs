use cargo_snippet::snippet;

#[snippet("binsearch")]
#[snippet(include = "binsearch_impl_[T]")]
#[snippet(include = "binsearch_impl_Range<i64>")]
trait BinarySearch {
    type Index;
    type Element;
    fn bisect<F>(&self, f: F) -> Self::Index
    where
        F: Fn(&Self::Element) -> bool;
}

#[snippet("binsearch_impl_[T]")]
impl<T> BinarySearch for [T] {
    type Index = i64;
    type Element = T;
    fn bisect<F>(&self, f: F) -> Self::Index
    where
        F: Fn(&Self::Element) -> bool,
    {
        let mut ng = -1;
        let mut ok = self.len() as i64;

        while (ok - ng).abs() > 1 {
            let mid = ok + (ng - ok) / 2;

            if f(&self[mid as usize]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        return ok;
    }
}

#[test]
fn test_binary_search() {
    assert_eq!(
        [10, 20, 30, 40, 50, 60, 70, 80, 90, 100].bisect(|&x| x >= 50),
        4
    );
    assert_eq!(
        vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100].bisect(|&x| x >= 50),
        4
    )
}

#[snippet("binsearch_impl_Range<i64>")]
impl BinarySearch for std::ops::Range<i64> {
    type Index = i64;
    type Element = i64;
    fn bisect<F>(&self, f: F) -> Self::Index
    where
        F: Fn(&Self::Element) -> bool,
    {
        let mut ng = self.start - 1;
        let mut ok = self.end;
        while (ok - ng).abs() > 1 {
            let mid = ok + (ng - ok) / 2;

            if f(&mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        return ok;
    }
}

#[test]
fn test_binary_search_range_i64() {
    assert_eq!((1..100).bisect(|&x| x * x >= 100), 10)
}
