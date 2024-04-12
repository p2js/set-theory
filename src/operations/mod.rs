use crate::sets::{FiniteSet, Set};

pub struct Union<'a, T> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<T>,
}
impl<T> Set<T> for Union<'_, T> {
    fn contains(&self, x: &T) -> bool {
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
impl<T> Set<T> for Intersection<'_, T> {
    fn contains(&self, x: &T) -> bool {
        self.a.contains(x) && self.b.contains(x)
    }
}
impl<'a, T> Intersection<'a, T> {
    pub fn of(a: &'a dyn Set<T>, b: &'a dyn Set<T>) -> Self {
        Self { a, b }
    }
}

pub struct Complement<'a, T> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<T>,
}
impl<T> Set<T> for Complement<'_, T> {
    fn contains(&self, x: &T) -> bool {
        self.a.contains(x) && !self.b.contains(x)
    }
}
impl<'a, T> Complement<'a, T> {
    pub fn of(a: &'a dyn Set<T>, b: &'a dyn Set<T>) -> Self {
        Self { a, b }
    }
}

pub struct CartesianProduct<'a, T, R> {
    a: &'a dyn Set<T>,
    b: &'a dyn Set<R>,
}
impl<T, R> Set<(T, R)> for CartesianProduct<'_, T, R> {
    fn contains(&self, x: &(T, R)) -> bool {
        self.a.contains(&x.0) && self.b.contains(&x.1)
    }
}
impl<'a, T, R> CartesianProduct<'a, T, R> {
    pub fn of(a: &'a dyn Set<T>, b: &'a dyn Set<R>) -> Self {
        Self { a, b }
    }
}

pub struct PowerSet<'a, T> {
    a: &'a dyn Set<T>,
}
impl<T> Set<FiniteSet<T>> for PowerSet<'_, T> {
    fn contains(&self, set: &FiniteSet<T>) -> bool {
        set.iter().all(|value| self.a.contains(value))
    }
}
impl<'a, T> PowerSet<'a, T> {
    pub fn of(a: &'a dyn Set<T>) -> Self {
        Self { a }
    }
}
