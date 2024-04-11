use set_theory::operations::*;
use set_theory::set::*;

fn main() {
    let a = FiniteSet::new(&[1, 2, 3]);
    let b = PredicateSet::new(&[|&x| x > 5]);

    let anb = Intersection::of(&a, &b);
    let aub = Union::of(&a, &b);

    println!("{}, {}", anb.contains(&2), aub.contains(&2));
}
