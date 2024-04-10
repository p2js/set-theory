use std::collections::HashSet;
use std::hash::Hash;

pub trait SetElement: Copy + PartialEq {}
impl<T> SetElement for T where T: Copy + PartialEq {}

pub trait Set<T: SetElement> {
    fn contains(&self, x: T) -> bool;
}

pub struct EmptySet {}
impl<T: SetElement> Set<T> for EmptySet {
    fn contains(&self, _x: T) -> bool {
        false
    }
}
impl EmptySet {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for EmptySet {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FiniteSet<T: SetElement> {
    values: HashSet<T>,
}
impl<T: SetElement> Set<T> for FiniteSet<T> {
    fn contains(&self, x: T) -> bool {
        self.values.iter().any(|v| *v == x)
    }
}
impl<T: SetElement + Eq + Hash> FiniteSet<T> {
    pub fn new(vals: &[T]) -> Self {
        Self {
            values: vals.iter().copied().collect(),
        }
    }
}

pub struct InfiniteSet<'a, T: SetElement> {
    predicates: &'a [fn(T) -> bool],
}
impl<T: SetElement> Set<T> for InfiniteSet<'_, T> {
    fn contains(&self, x: T) -> bool {
        self.predicates.iter().all(|p| p(x))
    }
}
impl<'a, T: SetElement> InfiniteSet<'a, T> {
    pub fn new(predicates: &'a [fn(T) -> bool]) -> Self {
        Self { predicates }
    }

    pub fn all() -> Self {
        Self {
            predicates: &[|_x| true],
        }
    }
}
