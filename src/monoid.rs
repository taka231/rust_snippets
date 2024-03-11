use cargo_snippet::snippet;

#[snippet("monoid")]
pub trait Monoid {
    fn id() -> Self;
    fn op(&self, b: &Self) -> Self;
}

#[snippet("max_monoid")]
#[derive(Clone, Debug, PartialEq)]
struct Max<T>(T);

#[snippet("max_monoid")]
macro_rules! max_monoid_derive_for_num {
    ($type: ty) => {
        impl Monoid for Max<$type> {
            fn id() -> Self {
                Max(<$type>::MIN)
            }
            fn op(&self, b: &Self) -> Self {
                Max(self.0.max(b.0))
            }
        }
        impl From<$type> for Max<$type> {
            fn from(x: $type) -> Self {
                Max(x)
            }
        }
    };
}

#[snippet("max_monoid")]
max_monoid_derive_for_num!(i32);
#[snippet("max_monoid")]
max_monoid_derive_for_num!(i64);
#[snippet("max_monoid")]
max_monoid_derive_for_num!(u32);
#[snippet("max_monoid")]
max_monoid_derive_for_num!(u64);
#[snippet("max_monoid")]
max_monoid_derive_for_num!(usize);

#[snippet("sum_monoid")]
#[derive(Clone, Debug, PartialEq)]
struct Sum<T>(T);

#[snippet("sum_monoid")]
macro_rules! sum_monoid_derive_for_num {
    ($type: ty) => {
        impl Monoid for Sum<$type> {
            fn id() -> Self {
                Sum(0)
            }
            fn op(&self, b: &Self) -> Self {
                Sum(self.0 + b.0)
            }
        }
        impl From<$type> for Sum<$type> {
            fn from(x: $type) -> Self {
                Sum(x)
            }
        }
    };
}

#[snippet("sum_monoid")]
sum_monoid_derive_for_num!(i32);
#[snippet("sum_monoid")]
sum_monoid_derive_for_num!(i64);
#[snippet("sum_monoid")]
sum_monoid_derive_for_num!(u32);
#[snippet("sum_monoid")]
sum_monoid_derive_for_num!(u64);
#[snippet("sum_monoid")]
sum_monoid_derive_for_num!(usize);

#[snippet("monoid_util")]
#[snippet(include = "max_monoid")]
#[snippet(include = "sum_monoid")]
/// select first
impl<T: Clone> Monoid for Option<T> {
    fn id() -> Self {
        None
    }
    fn op(&self, b: &Self) -> Self {
        match (self, b) {
            (Some(_), _) => self.clone(),
            _ => b.clone(),
        }
    }
}

#[snippet("map_monoid")]
pub trait MapMonoid: Monoid {
    type X: Monoid;
    fn mapping(&self, x: &Self::X) -> Self::X;
}

#[snippet("map_monoid_util")]
#[snippet(include = "add_action_monoid")]
#[snippet("option_map_monoid")]
impl<T: Monoid + Clone> MapMonoid for Option<T> {
    type X = T;
    fn mapping(&self, x: &Self::X) -> Self::X {
        match self {
            Some(val) => val.clone(),
            None => x.clone(),
        }
    }
}

#[snippet("add_action_monoid")]
struct AddAction<F, X>(F, std::marker::PhantomData<X>);
#[snippet("add_action_monoid")]
impl<F: Monoid, X: Monoid> Monoid for AddAction<F, X> {
    fn id() -> Self {
        AddAction(F::id(), std::marker::PhantomData)
    }
    fn op(&self, b: &Self) -> Self {
        AddAction(self.0.op(&b.0), std::marker::PhantomData)
    }
}
