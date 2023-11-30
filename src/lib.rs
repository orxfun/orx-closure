//! # orx_closure
//!
//! An explicit closure with absolute seperation of the captured data from the function.
//!
//! ## Why
//!
//! It aims to address certain issues observed working with the regular closures through `fn_traits` such as:
//!
//! * having to box closures without a good reason,
//! * having to add a generic parameters to structs holding closures without a good reason,
//! * impossibility (hopefully only for now) of returning a reference to the captured data.
//!
//!
//! ## A. Motivation
//!
//! Here we present some of the issues and the suggested solution with `Closure` type.
//!
//! ### A.1. `expected closure, found a different closure`
//!
//! We all observe this error messages at one point. This is due to the fact that every closure has a unique type. Further, these are compiler generated types which we cannot type out, instead we can type the `Fn` trait that they implement.
//!
//! This causes problems in higher order functions; or simply when conditionally assigning a function to a variable.
//!
//! Consider the following issue [https://github.com/rust-lang/rust/issues/87961](https://github.com/rust-lang/rust/issues/87961). The issue suggests to clarify the error message, but also demonstrates how we are not able to write a very simple function.
//!
//! ```rust ignore
//! fn returns_closure(hmm: bool, y: i32) -> impl Fn(i32) -> i32 {
//!     if hmm {
//!         move |x| x + y
//!     } else {
//!         move |x| x * y // doesn't compile!
//!     }
//! }
//!
//! let add_three = returns_closure(true, 3);
//! assert_eq!(42, add_three(39));
//!
//! let times_two = returns_closure(false, 2);
//! assert_eq!(42, times_two(21));
//! ```
//!
//! The above code won't compile because of the title of this section :)
//!
//! = note: expected closure `[closure@src\motiv.rs:6:17: 6:25]`
//!            found closure `[closure@src\motiv.rs:8:17: 8:25]`
//!
//! This is because **impl** in return position does not really allow generic returns. It allows us to return one concrete type that we cannot type, which is determined by the body of the function rather than the caller.
//!
//! Let's break the closure to its components:
//! * The captured data, say `Capture`. Here `Capture = i32`.
//! * The non-capturing function; i.e., a function pointer `fn` transforming an `In` to an `Out`, with additional access to the `Capture`. We can type it down as `fn(&Capture, In) -> Out`. Here it is `fn(&i32, i32) -> i32`.
//!
//! *The choice on the `&Capture` is intentional due to `Fn` being our favorite among `Fn`, `FnOnce` and `FnMut`. In other words, we want to be able to call the closure many times and we don't want to mutate. If we want to consume, we can simply capture by value.*
//!
//! If we consider the closure as the sum of these two components; or simply as the pair `(Capture, fn(&Capture, In) -> Out)`; it is clear that both if-else branches have the same type `(i32, fn(&i32, i32) -> i32)`, and hence, there is no reason to treat them as different types.
//!
//! This is exactly what the `Closure<Capture, In, Out>` struct does: it separates the captured data from the function pointer. Then, these functions become two different values of the same type, and hence, the following is valid.
//!
//!
//! ```rust
//! use orx_closure::*;
//!
//! fn returns_closure(hmm: bool, y: i32) -> Closure<i32, i32, i32> {
//!     if hmm {
//!         Capture(y).fun(|y, x| x + y)
//!     } else {
//!         Capture(y).fun(|y, x| x * y)
//!     }
//! }
//! let add_three = returns_closure(true, 3);
//! assert_eq!(42, add_three.call(39));
//!
//! let times_two = returns_closure(false, 2);
//! assert_eq!(42, times_two.call(21));
//! ```
//!
//! Even the following is allowed :)
//!
//! ```rust
//! use orx_closure::*;
//!
//! fn returns_closure(hmm: bool, y: i32) -> Closure<i32, i32, i32> {
//!     Capture(y).fun(if hmm { |y, x| x + y } else { |y, x| x * y })
//! }
//! ```
//!
//! The error message correctly says *no two closures, even if identical, have the same type*. But this is not a limitation for anonymous functions; they actually have the same type even if they are different as long as they don't capture the environment and have the same signature. `fn`s are cool.
//!
//! The following is a little more realistic example where we are able to nicely define the type of the closure with `Closure`:
//!
//! ```rust
//! use orx_closure::*;
//!     
//! fn create_closure(slice: &[i32], exclude_evens: bool) -> Closure<&[i32], i32, Option<i32>> {
//!     Capture(slice).fun(if exclude_evens {
//!         |x, lb| x.iter().filter(|&x| x % 2 == 1 && *x > lb).min().cloned()
//!     } else {
//!         |x, lb| x.iter().filter(|&x| *x > lb).min().cloned()
//!     })
//! }
//!
//! let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//!
//! let closure = create_closure(&numbers, true);
//! let fun = closure.as_fn(); // not to call 'call'
//!
//! assert_eq!(fun(1), Some(3));
//! assert_eq!(fun(5), None);
//! ```
//!
//!
//! #### Why not just `Box` it?
//!
//! It is true that additional direction solves most, but not all, of the problems mentioned here. For instance, the following code compiles and works just fine.
//!
//! ```rust
//! fn returns_closure(hmm: bool, y: i32) -> Box<dyn Fn(i32) -> i32> {
//!     if hmm {
//!         Box::new(move |x| x + y)
//!     } else {
//!         Box::new(move |x| x * y)
//! }
//! }
//! let add_three = returns_closure(true, 3);
//! assert_eq!(42, add_three(39));
//!
//! let times_two = returns_closure(false, 2);
//! assert_eq!(42, times_two(21));
//! ```
//!
//! The greatest power this brings is the ability to forget about the captured data. It is not in the signature, completely abstracted away. We don't know its size, we don't need to since it's now a trait object. However, it comes with certain drawbacks:
//!
//! * It adds the indirection. This will lead to additional allocation.
//! * Furthermore, and maybe more importantly, the possibility for compiler optimizations is significantly lower.
//! * And a matter of taste; the code becomes noisy with words `dyn`, `Box` and `Box::new` which has to be repeated in every branch.
//!
//! Also we are sometimes driven by a chain of events with such indirection:
//!
//! * As mentioned above, we notice we have to use a trait object, so we go with `Box<dyn Fn(i32) -> i32>`.
//! * As a first class citizen, we pass this function to another function as one of its arguments that is of generic type `F: Fn(i32) -> i32`.
//! * Everything works fine.
//! * At some point, we are required to easily and cheaply `Clone` ahd share the closure. Therefore, we change the indirection to be `Rc<dyn Fn(i32) -> i32>`.
//! * And suddenly, we cannot pass this closure to the other function since **`Fn<(i32,)>` is not implemented for `Rc<dyn Fn(i32) -> i32>`**.
//! * Not possible to pass the closure as a point-free value, we have to write another closure which does nothing but call this closure.
//! * Not a big deal, just an undesired compilation error.
//!
//!
//! ### A.2. Lifetimes!
//!
//! It is super easy to get the compiler angry with lifetimes when closures start returning references.
//!
//!
//! #### Simplest Closure to Return a Reference
//!
//! ... would be the one that returns a reference to the captured value. This might not look particularly useful but it is sufficient to demonstrate the problem.
//!
//! ```rust ignore
//! let x = 42;
//! let return_ref_to_capture = move || &x;
//! // <- won't compile: ^^ returning this value requires that `'1` must outlive `'2`
//! ```
//!
//!
//!
//! For different reasons, yet with the same error message, the following closure version won't compile either:
//!
//! ```rust ignore
//! let x = 42;
//! let return_ref_to_capture = Capture(x).fun(|x: &i32, _: ()| x); // <- won't compile
//! ```
//!
//! Lifetimes and elisions are complicated to have a single signature `fn(&Capture, In) -> Out` to work for all cases (*at least for now: [https://github.com/rust-lang/rfcs/pull/3216](https://github.com/rust-lang/rfcs/pull/3216).*).
//!
//! Therefore, we need to have a different signature, and hence a different struct called `ClosureRef`, for which the function pointer is `fn(&Capture, In) -> &Out`. This immediately solves all the problems, the following nicely works:
//!
//! ```rust
//! use orx_closure::*;
//!
//! let x = 42;
//! let return_ref_to_capture = Capture(x).fun_ref(|x: &i32, _: ()| x); // all good :)
//! ```
//!
//! #### How about Returning `M<&T>`?
//!
//! We did solve the problem to return a reference by using `ClosureRef`. However, we often return `Option<&Out>`. The return type itself is not a reference but has a lifetime related with the lifetime of the closure. Therefore, we need something else. Namely, `ClosureOptRef` which has the function pointer signature of `fn(&Capture, In) -> Option<&Out>`. Once we switch to this signature, everything again works.
//!
//! ```rust
//! use orx_closure::*;
//!
//! let x = 42;
//! let return_ref_to_capture = Capture(x).fun_option_ref(|x: &i32, _: ()| Some(x)); // all good :)
//! ```
//!
//! We also frequently return `Result<&Out, Error>` and for this purpose we have `ClosureResRef`:
//!
//! ```rust
//! use orx_closure::*;
//!
//! let x = 42;
//! let return_ref_to_capture: ClosureResRef<i32, (), i32, String> = // <- String is the error type here.
//!     Capture(x).fun_result_ref(|x: &i32, _: ()| Ok(x));
//! ```
//!
//! And this is where we should stop since it doesn't seem to be a good idea to enumerate all types with a wrapped reference `M<&T>`. We end up with four types with the following signatures:
//!
//!
//! | Closure Struct    | Data      | Function Pointer                          | Resulting `Fn` trait signature                     |
//! |-----------------------------------|-----------|------------------------------------|---------------------------------------------|
//! | `Closure<Capture, In, Out>`       | `Capture` | `fn(&Capture, In) -> Out`          | `Fn(In) -> Out` |
//! | `ClosureRef<Capture, In, Out>`    | `Capture` | `fn(&Capture, In) -> &Out`         | `Fn(In) -> &Out`         |
//! | `ClosureOptRef<Capture, In, Out>` | `Capture` | `fn(&Capture, In) -> Option<&Out>` | `Fn(In) -> Option<&Out>` |
//! | `ClosureResRef<Capture, In, Out, Error>` | `Capture` | `fn(&Capture, In) -> Result<&Out, Error>` | `Fn(In) -> Result<&Out, Error>` |
//!
//! *Hoping we eventually need only `Closure`.*
//!
//! ### A.3. Lifetimes when Captured by Ref
//!
//! The problems explained in **A.2**, leading us to implement four variants, are only relevant when we capture the data by value. The compiler allows us to represent all the abovementioned cases with the `Closure` signature:
//!
//! ```rust
//! use orx_closure::*;
//!
//! let x = 42;
//!
//! let closure_ref = Capture(&x).fun(|x, _: ()| *x);
//! assert_eq!(closure_ref.call(()), &42);
//!
//! let closure_opt_ref = Capture(&x).fun(|x, _: ()| Some(*x));
//! assert_eq!(closure_opt_ref.call(()), Some(&42));
//!
//! let closure_res_ref: Closure<_, _, Result<&i32, String>> = Capture(&x).fun(|x, _: ()| Ok(*x));
//! assert_eq!(closure_res_ref.call(()), Ok(&42));
//! ```
//!
//!
//! ## B. Abstraction over the Captured Data
//!
//! As mentioned before, using `dyn Fn(In) -> Out` trait object as closures has drawbacks of having to allocate and losing certain compiler optimization opportunities. However, it also provides the flexibility by allowing to forget about the captured data. This is actually one of the great things about closures, and hence, a requirement.
//!
//! On the other hand, `Closure` has to know the captured data, which is a big limitation.
//!
//! We can improve the situation to a certain extent by sum types. If the the closure will capture one of several possible types, then the closure could still be sized as an enum. However, we need to know all that can be used. This is not quiet the super power that `dyn Fn` has but covers a certain class of cases.
//!
//! ### A Practical Example
//!
//! Assume we want to have a closure to be used by some graph algorithm which would serve the queries *can node i precede node j* in an ordering. There might be many interesting variants and implementations, some are as follows:
//!
//! * Yes, every node can precede every other node. This is simply a function that returns `true` for all inputs (will hopefully be optimized away).
//! * We use a vector of sets of possible successors for each node, say `allowed: Vec<HashSet<Node>>`. Then, `i` can precede `j` only if `allowed[i].contains(j)`.
//! * There is a set of taboo pairs, say `taboo: HashSet<(Node, Node)>`. The answer is yes only if the pair does not exist in the taboo list.
//!
//! So we expect three captures to be relevant for our closure: `()`, `Vec<HashSet<Node>>` and `HashSet<(Node, Node)>`. Then, we can use `ClosureOneOf3`.
//!
//! ```rust
//! use orx_closure::*;
//! use std::collections::HashSet;
//!
//! type Node = usize; // for brevity
//! type Edge = (Node, Node); // for brevity
//!
//! type PrecedenceClosure = ClosureOneOf3<(), Vec<HashSet<Node>>, HashSet<Edge>, Edge, bool>;
//!
//! struct Precedence(PrecedenceClosure);
//!
//! impl Precedence {
//!     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
//!         Self(closure.into_oneof3_var1())
//!     }
//!     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
//!         Self(closure.into_oneof3_var2())
//!     }
//!     fn new_variant3(closure: Closure<HashSet<Edge>, Edge, bool>) -> Self {
//!         Self(closure.into_oneof3_var3())
//!     }
//!
//!     fn can_precede(&self, edge: Edge) -> bool {
//!         self.0.call(edge)
//!     }
//! }
//!
//! let allow_all = Precedence::new_variant1(Capture(()).fun(|_, _| true));
//! assert_eq!(allow_all.can_precede((0, 1)), true);
//! assert_eq!(allow_all.can_precede((10, 21)), true);
//!
//! let disallow_all = Precedence::new_variant1(Capture(()).fun(|_, _| false));
//! assert_eq!(disallow_all.can_precede((0, 1)), false);
//! assert_eq!(disallow_all.can_precede((10, 21)), false);
//!
//! let allowed: Vec<HashSet<Node>> = vec![
//!     HashSet::from_iter([1, 2, 3].into_iter()),
//!     HashSet::from_iter([2, 3].into_iter()),
//!     HashSet::from_iter([3].into_iter()),
//!     HashSet::from_iter([0].into_iter()),
//! ];
//! let from_allowed = Precedence::new_variant2(
//!     Capture(allowed).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
//! );
//! assert_eq!(from_allowed.can_precede((1, 3)), true);
//! assert_eq!(from_allowed.can_precede((2, 1)), false);
//!
//! let taboo = HashSet::from_iter([(0, 3), (1, 2)].into_iter());
//! let from_taboo =
//!     Precedence::new_variant3(Capture(taboo).fun(|taboo, edge| !taboo.contains(&edge)));
//! assert_eq!(from_taboo.can_precede((0, 3)), false);
//! assert_eq!(from_taboo.can_precede((2, 1)), true);
//! ```
//!
//! This approach has the following pros:
//!
//! * It is sized and easy to store in anywhere as a regular struct.
//! * It auto-implements `Clone` since all possible capture types implement `Clone`.
//! * `Precedence` struct does not need any generic parameter.
//!
//! And the following cons:
//!
//! * `ClosureOneOf3<C1, C2, C3, In, Out>` is a long type; hopefully, we only type it once.
//! * `Closure::into_oneof3_var1`, ``Closure::into_oneof3_var2``, etc. are type-safe and explicit functions, but not pretty.
//!
//! To sum up, once the ugliness is hidden in a small area, `Closur` provides a convenient third option between the two extemes:
//!
//! * having the closure as a generic parameter allowing monomorphization but adding a generic parameter to the parent, and
//! * having the closure as a `dyn Fn` trait object adding the indirection but not requiring the generic parameter.
//!
//! This middle ground fits well with closures having specific functionalities such as the `Precedence`.
//!
//! ## C. Relation with `Fn` trait
//!
//! Note that `Closure<Capture, In, Out>` has the method `fn call(&self, input: In) -> Out`. Therefore, it could have implemented `Fn(In) -> Out`. But the compiler tells me that *manual implementations of `Fn` are experimental*, and adds the error *use of unstable library feature 'fn_traits'*. Not wanting to be unstable, `Clsoure` does not implement the `Fn` trait.
//!
//! Instead, `Closure` and all relevant types have the `as_fn` method such as `fn as_fn(&self) -> impl Fn(In) -> Out + '_ ` which gives us the compiler generated closure implementing the `Fn` trait.
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod capture;
mod closure_opt_ref;
mod closure_ref;
mod closure_res_ref;
mod closure_val;
mod one_of;
mod one_of_variants;

pub use capture::Capture;
pub use closure_opt_ref::ClosureOptRef;
pub use closure_ref::ClosureRef;
pub use closure_res_ref::ClosureResRef;
pub use closure_val::Closure;
pub use one_of::{OneOf2, OneOf3, OneOf4};

pub use one_of_variants::one_of2::{
    closure_opt_ref::ClosureOptRefOneOf2, closure_ref::ClosureRefOneOf2,
    closure_res_ref::ClosureResRefOneOf2, closure_val::ClosureOneOf2,
};

pub use one_of_variants::one_of3::{
    closure_opt_ref::ClosureOptRefOneOf3, closure_ref::ClosureRefOneOf3,
    closure_res_ref::ClosureResRefOneOf3, closure_val::ClosureOneOf3,
};

pub use one_of_variants::one_of4::{
    closure_opt_ref::ClosureOptRefOneOf4, closure_ref::ClosureRefOneOf4,
    closure_res_ref::ClosureResRefOneOf4, closure_val::ClosureOneOf4,
};
