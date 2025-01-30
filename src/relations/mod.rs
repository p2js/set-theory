use crate::sets::Set;

pub struct Relation<'a, S: Set<T>, T> {
    set: &'a S,
    relation_predicate: fn(&T, &T) -> bool,
}

impl<'a, S: Set<T>, T> Relation<'a, S, T> {
    pub fn on(set: &'a S, relation_predicate: fn(&T, &T) -> bool) -> Self {
        Self {
            set,
            relation_predicate,
        }
    }

    pub fn relates(&self, a: &T, b: &T) -> bool {
        self.set.contains(a) && self.set.contains(b) && (self.relation_predicate)(a, b)
    }
}
