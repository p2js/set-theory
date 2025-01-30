use set_theory::operations::*;
use set_theory::relations::Relation;
use set_theory::sets::*;

fn main() {
    let a = FiniteSet::new(&[1, 2, 3]);
    let b = PredicateSet::new(&[|&x| x > 5, |&x| x % 2 == 0]);

    let e = EmptySet::new();
    assert!(!e.contains(&0));
    assert!(!e.contains(&1.5));

    assert!(b.contains(&6));
    assert!(!b.contains(&7));

    let anb = Intersection::of(&a, &b);
    let aub = Union::of(&a, &b);

    assert!(aub.contains(&2));
    assert!(!anb.contains(&2));

    let pa = PowerSet::of(&a);
    assert!(pa.contains(&a));
    assert!(pa.contains(&FiniteSet::new(&[1])));

    let c = PredicateSet::<i32>::all();

    let equivalence_relation = Relation::on(&c, |a, b| a % 2 == b % 2);

    assert!(equivalence_relation.relates(&2, &4));
    assert!(!equivalence_relation.relates(&2, &3));
}
