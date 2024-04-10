use crate::set::{Set, SetElement};

pub struct Union<'a, T> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<T>,
}
impl<T: SetElement> Set<T> for Union<'_, T> {
    fn contains(&self, x: T) -> bool {
        self.a.contains(x) || self.b.contains(x)
    }
}
impl<'a, T> Union<'a, T> {
    pub fn of(a: &'a dyn Set<T>, b: &'a dyn Set<T>) -> Self {
        Self { a, b }
    }
}

pub struct Intersection<'a, T> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<T>,
}
impl<T: SetElement> Set<T> for Intersection<'_, T> {
    fn contains(&self, x: T) -> bool {
        self.a.contains(x) && self.b.contains(x)
    }
}
impl<'a, T> Intersection<'a, T> {
    pub fn of(a: &'a dyn Set<T>, b: &'a dyn Set<T>) -> Self {
        Self { a, b }
    }
}

pub struct CartesianProduct<'a, T, R> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<R>,
}
impl<T: SetElement, R: SetElement> Set<(T, R)> for CartesianProduct<'_, T, R> {
    fn contains(&self, x: (T, R)) -> bool {
        self.a.contains(x.0) && self.b.contains(x.1)
    }
}
