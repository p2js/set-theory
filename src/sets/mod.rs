use std::collections::HashSet;
use std::hash::Hash;

pub trait Set<T> {
    fn contains(&self, x: &T) -> bool;
}

pub struct EmptySet {}
impl<T> Set<T> for EmptySet {
    fn contains(&self, _x: &T) -> bool {
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

pub struct FiniteSet<T> {
    values: HashSet<T>,
}
impl<T: PartialEq> Set<T> for FiniteSet<T> {
    fn contains(&self, x: &T) -> bool {
        self.values.iter().any(|v| *v == *x)
    }
}
impl<T: Copy + Eq + Hash> FiniteSet<T> {
    pub fn new(vals: &[T]) -> Self {
        Self {
            values: vals.iter().copied().collect(),
        }
    }
}
impl<T> FiniteSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.values.iter()
    }
}

pub struct PredicateSet<'a, T> {
    predicates: &'a [fn(&T) -> bool],
}
impl<T> Set<T> for PredicateSet<'_, T> {
    fn contains(&self, x: &T) -> bool {
        self.predicates.iter().all(|p| p(x))
    }
}
impl<'a, T> PredicateSet<'a, T> {
    pub fn new(predicates: &'a [fn(&T) -> bool]) -> Self {
        Self { predicates }
    }

    pub fn all() -> Self {
        Self {
            predicates: &[|_x| true],
        }
    }
}
