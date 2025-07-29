# orx_closure

An explicit closure with absolute separation of the captured data from the function.

## Why

It aims to address certain issues observed working with the regular closures through `fn_traits` such as:

* having to box closures without a good reason,
* having to add generic parameters to structs holding closures without a good reason,
* impossibility (hopefully only for now) of returning a reference to the captured data.


## A. Motivation

Some of the mentioned issues and suggested solution with `Closure` type are discussed here.

### A.1. `expected closure, found a different closure`

We all observe this error messages at one point. This is due to the fact that every closure has a unique type. Further, these are compiler generated types which we cannot type out, instead we can type the `Fn` trait that they implement.

This causes problems in higher order functions; or simply when conditionally assigning a function to a variable.

Consider the following issue [https://github.com/rust-lang/rust/issues/87961](https://github.com/rust-lang/rust/issues/87961). The issue suggests to clarify the error message, but also demonstrates how we are not able to write a very simple function.

```rust ignore
fn returns_closure(hmm: bool, y: i32) -> impl Fn(i32) -> i32 {
    if hmm {
        move |x| x + y
    } else {
        move |x| x * y // doesn't compile!
    }
}

let add_three = returns_closure(true, 3);
assert_eq!(42, add_three(39));

let times_two = returns_closure(false, 2);
assert_eq!(42, times_two(21));
```

The above code won't compile because of the title of this section :)

  = note: expected closure `[closure@src\motiv.rs:6:17: 6:25]`
             found closure `[closure@src\motiv.rs:8:17: 8:25]`

Because **impl** in return position does not allow generic returns. It allows us to return one concrete type that we cannot type, which is determined by the body of the function rather than the caller.

Let's break the closure to its components:
* The captured data, say `Capture`. Here `Capture = i32`.
* The non-capturing function; i.e., a function pointer `fn` transforming an `In` to an `Out`, with additional access to the `Capture`. We can type it down as `fn(&Capture, In) -> Out`. Here it is `fn(&i32, i32) -> i32`.

*The choice on the `&Capture` is intentional due to `Fn` being absolute favorite among `Fn`, `FnOnce` and `FnMut`. In other words, we want to be able to call the closure many times and we don't want to mutate. If we want to consume, we can simply capture by value; recall that the data and `fn` are separated.* 

If we consider the closure as the product of these two components; or simply as the pair `(Capture, fn(&Capture, In) -> Out)`; it is clear that both if-else branches have the same type `(i32, fn(&i32, i32) -> i32)`, there is no reason to treat them as different types.

This is exactly what the `Closure<Capture, In, Out>` struct does: it separates the captured data from the function pointer. Then, these functions become two different values of the same type, and hence, the following is valid.


```rust
use orx_closure::*;

fn returns_closure(hmm: bool, y: i32) -> Closure<i32, i32, i32> {
    if hmm {
        Capture(y).fun(|y, x| x + y)
    } else {
        Capture(y).fun(|y, x| x * y)
    }
}
let add_three = returns_closure(true, 3);
assert_eq!(42, add_three.call(39));

let times_two = returns_closure(false, 2);
assert_eq!(42, times_two.call(21));
```

Even the following is allowed :)

```rust
use orx_closure::*;

fn returns_closure(hmm: bool, y: i32) -> Closure<i32, i32, i32> {
    Capture(y).fun(if hmm { |y, x| x + y } else { |y, x| x * y })
}
```

The error message correctly says *no two closures, even if identical, have the same type*. But this is not a limitation for anonymous functions; they actually have the same type even if they are different as long as they don't capture the environment and have the same signature. `fn`s are nice.

The following is a little more realistic example where we are able to nicely define the type of the `Closure`:

```rust
use orx_closure::*;
    
fn create_closure(slice: &[i32], exclude_evens: bool) -> Closure<&[i32], i32, Option<i32>> {
    Capture(slice).fun(if exclude_evens {
        |x, lb| x.iter().filter(|&x| x % 2 == 1 && *x > lb).min().cloned()
    } else {
        |x, lb| x.iter().filter(|&x| *x > lb).min().cloned()
    })
}

let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

let closure = create_closure(&numbers, true);
let fun = closure.as_fn(); // not to call 'call'

assert_eq!(fun(1), Some(3));
assert_eq!(fun(5), None);
```


#### Why not just `Box` it?

It is true that additional indirection solves not all but most of the problems mentioned here. For instance, the following code compiles and works just fine.

```rust
fn returns_closure(hmm: bool, y: i32) -> Box<dyn Fn(i32) -> i32> {
    if hmm {
        Box::new(move |x| x + y)
    } else {
        Box::new(move |x| x * y)
    }
}
let add_three = returns_closure(true, 3);
assert_eq!(42, add_three(39));

let times_two = returns_closure(false, 2);
assert_eq!(42, times_two(21));
```

The greatest power this brings is the ability to forget about the captured data. It is not in the signature, completely abstracted away. We don't know its size, we don't need to since it's now a trait object. However, it comes with certain drawbacks:

* It adds the indirection. This will lead to additional allocation.
* Furthermore, and maybe more importantly, the possibility for compiler optimizations is significantly lower.
* And a matter of taste; the code becomes noisy with words `dyn`, `Box` and `Box::new` which has to be repeated in every branch.

Also we are sometimes driven by a chain of events with such indirection:

* As mentioned above, we notice we have to use a trait object, so we go with `Box<dyn Fn(i32) -> i32>`.
* As a first class citizen, we pass this function to another function as one of its arguments that is of generic type `F: Fn(i32) -> i32`.
* Everything works fine.
* At some point, we are required to easily and cheaply `Clone` and share the closure. Therefore, we change the indirection to be `Rc<dyn Fn(i32) -> i32>`.
* And suddenly, we cannot pass this closure to the other function since **`Fn<(i32,)>` is not implemented for `Rc<dyn Fn(i32) -> i32>`**.
* Not possible to pass the closure as a point-free value.
* We sadly write another closure which does nothing but call this closure.
* Not a big deal, but makes you ask why.


### A.2. Lifetimes!

It is super easy to get the compiler angry with lifetimes when closures start returning references.


#### Simplest Closure to Return a Reference

... would be the one that returns a reference to the captured value. This might not look particularly useful but it actually is useful to demonstrate the problem.

```rust ignore
let x = 42;
let return_ref_to_capture = move || &x;
// <- won't compile: ^^ returning this value requires that `'1` must outlive `'2`
```

For different reasons, yet with the same error message, the following closure version won't compile either:

```rust ignore
use orx_closure::*;

let x = 42;
let return_ref_to_capture = Capture(x).fun(|x: &i32, _: ()| x); // <- won't compile
```

Lifetimes and elisions are complicated to have a single signature `fn(&Capture, In) -> Out` to work for all cases (*at least for now: [https://github.com/rust-lang/rfcs/pull/3216](https://github.com/rust-lang/rfcs/pull/3216).*).

Therefore, we need to have a different signature, and hence a different struct called `ClosureRef`, for which the function pointer is `fn(&Capture, In) -> &Out`. This immediately solves all the problems, the following nicely works:

```rust
use orx_closure::*;

let x = 42;
let return_ref_to_capture = Capture(x).fun_ref(|x: &i32, _: ()| x); // all good :)
```

#### How about Returning `M<&T>`?

We did solve the problem to return a reference by using `ClosureRef`. However, we often return `Option<&Out>`. The return type itself is not a reference but has a lifetime related with the lifetime of the closure. Therefore, we need something else. Namely, `ClosureOptRef` which has the function pointer signature of `fn(&Capture, In) -> Option<&Out>`. Once we switch to this signature, everything again works.

```rust
use orx_closure::*;

let x = 42;
let return_ref_to_capture = Capture(x).fun_option_ref(|x: &i32, _: ()| Some(x)); // all good :)
```

We also frequently return `Result<&Out, Error>` and for this purpose we have `ClosureResRef`:

```rust
use orx_closure::*;

let x = 42;
let return_ref_to_capture: ClosureResRef<i32, (), i32, String> = // <- String is the error type here.
    Capture(x).fun_result_ref(|x: &i32, _: ()| Ok(x));
```

And this is where we should stop since it doesn't seem to be a good idea to enumerate all types with a wrapped reference `M<&T>`. We end up with four types with the following signatures:


| Closure Struct    | Data      | Function Pointer                          | Resulting `Fn` trait signature                     |
|-----------------------------------|-----------|------------------------------------|---------------------------------------------|
| `Closure<Capture, In, Out>`       | `Capture` | `fn(&Capture, In) -> Out`          | `Fn(In) -> Out` |
| `ClosureRef<Capture, In, Out>`    | `Capture` | `fn(&Capture, In) -> &Out`         | `Fn(In) -> &Out`         |
| `ClosureOptRef<Capture, In, Out>` | `Capture` | `fn(&Capture, In) -> Option<&Out>` | `Fn(In) -> Option<&Out>` |
| `ClosureResRef<Capture, In, Out, Error>` | `Capture` | `fn(&Capture, In) -> Result<&Out, Error>` | `Fn(In) -> Result<&Out, Error>` |

It is straightforward to decide which closure variant to use:

* If we capture the data by reference, `Capture(&data)`, we can use `Closure` for any return type.
* If we return a value that does not have a lifetime related to the closure, we can again use `Closure` independent of how we captured the data.
* However, if we capture the data with its ownership, `Capture(data)`, and want to return a value lifetime of which depends on the lifetime of the closure:
  * we use `ClosureRef` if we want to return `&Out`,
  * we use `ClosureOptRef` if we want to return `Option<&Out>`,
  * we use `ClosureResRef` if we want to return `Result<&Out, _>`.

*Hoping we eventually need only `Closure`.*

### A.3. Lifetimes when Captured by Ref

The problems explained in **A.2**, leading us to implement four variants, are only relevant when we capture the data by value. The compiler allows us to represent all the above-mentioned cases with the `Closure` signature:

```rust
use orx_closure::*;

let x = 42;

let closure_ref = Capture(&x).fun(|x, _: ()| *x);
assert_eq!(closure_ref.call(()), &42);

let closure_opt_ref = Capture(&x).fun(|x, _: ()| Some(*x));
assert_eq!(closure_opt_ref.call(()), Some(&42));

let closure_res_ref: Closure<_, _, Result<&i32, String>> = Capture(&x).fun(|x, _: ()| Ok(*x));
assert_eq!(closure_res_ref.call(()), Ok(&42));
```


## B. Abstraction over the Captured Data

As mentioned before, using `dyn Fn(In) -> Out` trait object as closures has drawbacks of having to allocate and losing certain compiler optimization opportunities. However, it also provides the flexibility by allowing to forget the captured data. This is actually one of the main reasons why closures are so useful.

On the other hand, `Closure` has to know the captured data, which is a big limitation.

We can improve the situation to a certain extent by sum types as follows.

If the the closure will capture one of several possible types, then the closure could still be sized as an enum. However, we need to know all that can be used. This is not quiet the super power that `dyn Fn` has but covers a certain class of cases.

### A Practical Example

Assume we want to have a closure to be used by some graph algorithm which would serve the queries *can node i precede node j* in an ordering. There might be many interesting variants and implementations, some are as follows:

* Yes, every node can precede every other node. This is simply a function that returns `true` for all inputs (will hopefully be optimized away).
* We use a vector of sets of possible successors for each node, say `allowed: Vec<HashSet<Node>>`. Then, `i` can precede `j` only if `allowed[i].contains(j)`.
* There is a set of taboo pairs, say `taboo: HashSet<(Node, Node)>`. The answer is yes only if the pair does not exist in the taboo list.

So we expect three captures to be relevant for our closure: `()`, `Vec<HashSet<Node>>` and `HashSet<(Node, Node)>`. Then, we can use `ClosureOneOf3`.

```rust
use orx_closure::*;
use std::collections::HashSet;

type Node = usize; // for brevity
type Edge = (Node, Node); // for brevity

type PrecedenceClosure = ClosureOneOf3<(), Vec<HashSet<Node>>, HashSet<Edge>, Edge, bool>;

struct Precedence(PrecedenceClosure);

impl Precedence {
    fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
        Self(closure.into_oneof3_var1())
    }
    fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
        Self(closure.into_oneof3_var2())
    }
    fn new_variant3(closure: Closure<HashSet<Edge>, Edge, bool>) -> Self {
        Self(closure.into_oneof3_var3())
    }

    fn can_precede(&self, edge: Edge) -> bool {
        self.0.call(edge)
    }
}

let allow_all = Precedence::new_variant1(Capture(()).fun(|_, _| true));
assert_eq!(allow_all.can_precede((0, 1)), true);
assert_eq!(allow_all.can_precede((10, 21)), true);

let disallow_all = Precedence::new_variant1(Capture(()).fun(|_, _| false));
assert_eq!(disallow_all.can_precede((0, 1)), false);
assert_eq!(disallow_all.can_precede((10, 21)), false);

let allowed: Vec<HashSet<Node>> = vec![
    HashSet::from_iter([1, 2, 3].into_iter()),
    HashSet::from_iter([2, 3].into_iter()),
    HashSet::from_iter([3].into_iter()),
    HashSet::from_iter([0].into_iter()),
];
let from_allowed = Precedence::new_variant2(
    Capture(allowed).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
);
assert_eq!(from_allowed.can_precede((1, 3)), true);
assert_eq!(from_allowed.can_precede((2, 1)), false);

let taboo = HashSet::from_iter([(0, 3), (1, 2)].into_iter());
let from_taboo =
    Precedence::new_variant3(Capture(taboo).fun(|taboo, edge| !taboo.contains(&edge)));
assert_eq!(from_taboo.can_precede((0, 3)), false);
assert_eq!(from_taboo.can_precede((2, 1)), true);
```

This approach has the following pros:

* It is sized and easy to store in anywhere as a regular struct.
* It auto-implements `Clone` since all possible capture types implement `Clone`.
* `Precedence` struct does not need any generic parameter.

And the following cons:

* `ClosureOneOf3<C1, C2, C3, In, Out>` is a long type; hopefully, we only type it once.
* `Closure::into_oneof3_var1`, ``Closure::into_oneof3_var2``, etc. are type-safe and explicit functions, but not pretty.

To sum up, once the ugliness is hidden in a small box, `Closure` provides a convenient third option between the two extremes:

* having the closure as a generic parameter allowing monomorphization but adding a generic parameter to the parent, and
* having the closure as a `dyn Fn` trait object adding the indirection but not requiring the generic parameter.

This middle ground fits well with closures having specific functionalities such as the `Precedence`.

## C. Abstraction over the Captured Data with Trait Objects

We are not able to implement `fn_traits` in stable rust; however as discussed, abstraction over the captured data type is the core power of closures. In order to achieve this flexibility, this crate provides the required traits `Fun`, `FunRef`, `FunOptRef` and `FunResRef`. The following table provides the complete list of traits and types implementing them.

| Trait                       | Transformation              | Struct                                                |
|-----------------------------|-----------------------------|-------------------------------------------------------|
| `Fun<In, Out>`              | `In -> Out`                 | `T where T: Fn(In) -> Out`                            |
|                             |                             | `Closure<Capture, In, Out>`                           |
|                             |                             | `ClosureOneOf2<C1, C2, In, Out>`                      |
|                             |                             | `ClosureOneOf3<C1, C2, C3, In, Out>`                  |
|                             |                             | `ClosureOneOf4<C1, C2, C3, C4, In, Out>`              |
| `FunRef<In, Out>`           | `In -> &Out`                | `ClosureRef<Capture, In, Out>`                        |
|                             |                             | `ClosureRefOneOf2<C1, C2, In, Out>`                   |
|                             |                             | `ClosureRefOneOf3<C1, C2, C3, In, Out>`               |
|                             |                             | `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>`           |
| `FunOptRef<In, Out>`        | `In -> Option<&Out>`        | `ClosureOptRef<Capture, In, Out>`                     |
|                             |                             | `ClosureOptRefOneOf2<C1, C2, In, Out>`                |
|                             |                             | `ClosureOptRefOneOf3<C1, C2, C3, In, Out>`            |
|                             |                             | `ClosureOptRefOneOf4<C1, C2, C3, C4, In, Out>`        |
| `FunResRef<In, Out, Error>` | `In -> Result<&Out, Error>` | `ClosureResRef<Capture, In, Out, Error>`              |
|                             |                             | `ClosureResRefOneOf2<C1, C2, In, Out, Error>`         |
|                             |                             | `ClosureResRefOneOf3<C1, C2, C3, In, Out, Error>`     |
|                             |                             | `ClosureResRefOneOf4<C1, C2, C3, C4, In, Out, Error>` |

The fun traits are useful due to the following:

* The are to be used as a generic parameter which can be filled up by any of the implementing types. This is exactly the purpose of the `Fn` traits. The two reasons why we don't directly use the `Fn` trait is as follows:
  * We are not allowed to implement `Fn` trait in stable rust for the `Capture` types defined in this crate.
  * Apart from `Fun`, we are not able to represent the reference-returning traits with the `Fn` trait due to lifetime errors.
* They allow to create trait objects from the closures, such as `dyn Fun<In, Out>`, etc., whenever we do not (want to) know the capture type.


## D. Relation with `Fn` trait

Note that `Closure<Capture, In, Out>` has the method `fn call(&self, input: In) -> Out`. Therefore, it could have implemented `Fn(In) -> Out`. But the compiler tells me that *manual implementations of `Fn` are experimental*, and adds the *use of unstable library feature 'fn_traits'* error. Not wanting to be unstable, `Closure` does not implement the `Fn` trait.

Instead, `Closure` and all variants have the `as_fn` method, such as `fn as_fn(&self) -> impl Fn(In) -> Out + '_ `, which gives us the compiler generated closure implementing the `Fn` trait.

## E. Benchmarks & Performance

Assume we have the requirement to hold a function as a field of a struct. In the example case defined in `/benches/fun_as_a_field`, we hold the function that accesses two-index access to a jagged array.

The variants look like below:

```rust
use orx_closure::*;

type Weight = i32;
type WithClosure<Weight> = Closure<Vec<Vec<Weight>>, (usize, usize), Weight>; // no generics required

struct HoldingFn<F: Fn((usize, usize)) -> Weight> { // requires the generic parameter F
    fun: F,
}

struct HoldingBoxDynFn { // no generics required
    fun: Box<dyn Fn((usize, usize)) -> Weight>,
}
```

And the results are as follows:

```ignore
FunAsAField/closure/10000
                        time:   [126.07 ms 126.63 ms 127.23 ms]
FunAsAField/holding_fn/10000
                        time:   [55.149 ms 55.372 ms 55.604 ms]
FunAsAField/holding_box_dyn_fn/10000
                        time:   [127.27 ms 128.24 ms 129.40 ms]
```

As expected, holding the closure as a generic field implementing the `Fn` performs the best. The generic parameter allows for monomorphization and compiler optimizations. However, not any two instances of the `HoldingFn` struct will have the same type.

`Closure` and `Box<dyn Fn ...>` approaches perform equivalently, which is slightly slower than twice the generic version. This shows us that neither of them can benefit from certain inlining optimizations. It is due to the fact that we can have different versions/implementations of a function only if it has generic parameters. On the other hand, with `Closure`, we treat closures as different values of the same type without the need for a generic parameter. Then, the implementation of the function to which we pass the closure is the general one which calls the closure through the function pointer, without any opportunity to inline.

From another perspective, it is actually surprising that `Closure` and `Box<dyn Fn ...>` are only ~two times slower than probably completely inlined version of a very small function which does nothing but `data[i][j]` where `data: Vec<Vec<i32>>`.

Nevertheless, to sum up:

* in performance-critical cases we would prefer to use the generic parameter & `impl Fn` approach to get the best performance;
* however, we can prefer  `Box<dyn Fn ...>` or `Closure` approaches when the job done by the closure is large enough to make the indirection insignificant:
  * for instance, the indirection will be significant if all the closure does is to allow access to data as we saw in the benchmark,
  * however, will most certainly be insignificant if it performs matrix multiplication.

## F. Final Remarks

The benchmark above sort of settles down the use cases:

* When we are okay to add the generic parameter and when we are not returning a reference to the captured data, `impl Fn` is the most performant option and does not require heap allocation.
* Otherwise:
  * `Box<dyn Fn ...>`:
    * does not require a generic parameter,
    * is as flexible as it could be in abstraction over the captured data by completely hiding it,
    * however, requires heap allocation,
    * we experience lifetime issues when returning a reference to the captured data.
  * `Closure`
    * does not require a generic parameter,
    * has to remember the captured data type: it allows abstraction over the captured data to some extent; however, not so nicely and magically as `Fn` traits do,
    * does not require heap allocation,
    * auto-implements `Clone` provided that the captured data implements `Clone`,
    * solves a useful set of lifetime issues that we cannot solve with `Fn` traits.

*Note: This crate has been an experiment of a very simple idea, which helped me understand the underlying magic as well as current limitations of rust closures much better. Probably there is still a lot more to figure out. Nevertheless, it ended up with the `Closure` struct which is what I needed for another problem (actually, it was why I started playing around at the first place). In brief, issues, comments, corrections, suggestions, interesting ideas to experiment are all very welcome.*

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
