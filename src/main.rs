use set_rs::operations::*;
use set_rs::set::*;

fn main() {
    let a = FiniteSet::new(&[1, 2, 3]);
    let b = InfiniteSet::new(&[|x| x > 5]);

    let anb = Intersection::of(&a, &b);
    let aub = Union::of(&a, &b);

    println!("{}, {}", anb.contains(2), aub.contains(2));
}
