# set-theory

Type-system set theory in rust. 

This is a quick project thrown together in a couple hours to explore the representation of infinite sets and interoperability between different set types.

## The Empty set

The `EmptySet` type represents an empty set. It is simply an empty `struct` that implements the `Set<T>` trait.

```rs
use set_theory::sets::EmptySet;

let es = EmptySet::new();

assert!(!es.contains(&0));
assert!(!es.contains(&1.5));
```

## Finite and Infinite sets

The `FiniteSet<T>` type can store a set of finite values, and uses a `HashSet<T>` internally.

```rs
use set_theory::sets::FiniteSet;

let fs = FiniteSet::new(&[1, 2, 3]);

assert!(fs.contains(1));
assert!(!fs.contains(4));
```

The `PredicateSet<T>` is meant to represent an infinite set as a list of predicates (functions that take in a single `T` argument and return a boolean), in a sort of emulation of set builder notation. A value will count as being in the set if all the predicates evaluate to true.

```rs
use set_theory::sets::PredicateSet;

// set of all even integers greater than 5
let ps = PredicateSet::new(&[|&x| x > 5, |&x| x % 2 == 0]);

assert!(ps.contains(&6));
assert!(!ps.contains(&7));
```

## Set operations

The library includes types for the unions, intersections, complements and cartesian products of two sets. It also includes a power set type, which will implement `Set<FiniteSet<T>>` for any `Set<T>` and will check for whether a set is in the powerset by checking that all the elements are contained in the original set.

All of these types use dynamic trait object references to allow for operations on different types of sets.

```rs
use set_theory::operations::{Union, PowerSet};
use set_theory::sets::{FiniteSet, PredicateSet};

let a = FiniteSet::new(&[1, 5]);
let b = PredicateSet::new(&[|&x| x > 5, |&x| x % 2 == 0]);

let aub = Union::of(&a, &b);
assert!(aub.contains(&5));
assert!(aub.contains(&6));

let pa = PowerSet::of(&a);
assert!(pa.contains(&a));
assert!(pa.contains(&FiniteSet::new(&[1])));
```

## Interacting with sets

All set types in the library implement a common `Set<T>` trait. The trait only requires implementation of a function to check whether a value is in the set. This is because infinite sets do not store any values, so any method to access some list of values would be extremely difficult (to impossible).