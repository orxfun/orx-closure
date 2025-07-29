use crate::{Closure, OneOf2, fun::Fun};

/// `ClosureOneOf2<C1, C2, In, Out>` is a union of two closures:
///
/// * `Closure<C1, In, Out>`
/// * `Closure<C2, In, Out>`
///
/// This is useful when it is possible that the closure might capture and work with either of the two types of data `C1` and `C2`.
///
/// It represents the transformation `In -> Out`.
///
/// Note that, unlike trait objects of fn-traits, `ClosureOneOf2` auto-implements `Clone` given that captured data variants are cloneable.
///
/// # Example
///
/// ```rust
/// use orx_closure::*;
/// use std::collections::HashSet;
///
/// type Node = usize; // for brevity
/// type Edge = (Node, Node); // for brevity
///
/// // captures either () or Vec<HashSet<Node>>
/// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
///
/// struct Precedence(PrecedenceClosure);
///
/// impl Precedence {
///     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
///         Self(closure.into_oneof2_var1())
///     }
///     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
///         Self(closure.into_oneof2_var2())
///     }
///
///     fn can_precede(&self, edge: Edge) -> bool {
///         self.0.call(edge)
///     }
/// }
///
/// let allow_all = Precedence::new_variant1(Capture(()).fun(|_, _| true));
/// assert_eq!(allow_all.can_precede((0, 1)), true);
/// assert_eq!(allow_all.can_precede((10, 21)), true);
///
/// let disallow_all = Precedence::new_variant1(Capture(()).fun(|_, _| false));
/// assert_eq!(disallow_all.can_precede((0, 1)), false);
/// assert_eq!(disallow_all.can_precede((10, 21)), false);
///
/// let allowed: Vec<HashSet<Node>> = vec![
///     HashSet::from_iter([1, 2, 3].into_iter()),
///     HashSet::from_iter([2, 3].into_iter()),
///     HashSet::from_iter([3].into_iter()),
///     HashSet::from_iter([0].into_iter()),
/// ];
/// let from_allowed = Precedence::new_variant2(
///     Capture(allowed).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
/// );
/// assert_eq!(from_allowed.can_precede((1, 3)), true);
/// assert_eq!(from_allowed.can_precede((2, 1)), false);
/// ```
#[derive(Clone, Debug)]
pub struct ClosureOneOf2<C1, C2, In, Out> {
    closure: OneOf2<Closure<C1, In, Out>, Closure<C2, In, Out>>,
}
impl<C1, C2, In, Out> ClosureOneOf2<C1, C2, In, Out> {
    /// Calls the closure with the given `input`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    /// use std::collections::HashSet;
    ///
    /// type Node = usize; // for brevity
    /// type Edge = (Node, Node); // for brevity
    ///
    /// // captures either () or Vec<HashSet<Node>>
    /// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
    ///
    /// struct Precedence(PrecedenceClosure);
    ///
    /// impl Precedence {
    ///     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var1())
    ///     }
    ///     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var2())
    ///     }
    ///
    ///     fn can_precede(&self, edge: Edge) -> bool {
    ///         self.0.call(edge)
    ///     }
    /// }
    ///
    /// let allow_all = Precedence::new_variant1(Capture(()).fun(|_, _| true));
    /// assert_eq!(allow_all.can_precede((0, 1)), true);
    /// assert_eq!(allow_all.can_precede((10, 21)), true);
    ///
    /// let disallow_all = Precedence::new_variant1(Capture(()).fun(|_, _| false));
    /// assert_eq!(disallow_all.can_precede((0, 1)), false);
    /// assert_eq!(disallow_all.can_precede((10, 21)), false);
    ///
    /// let allowed: Vec<HashSet<Node>> = vec![
    ///     HashSet::from_iter([1, 2, 3].into_iter()),
    ///     HashSet::from_iter([2, 3].into_iter()),
    ///     HashSet::from_iter([3].into_iter()),
    ///     HashSet::from_iter([0].into_iter()),
    /// ];
    /// let from_allowed = Precedence::new_variant2(
    ///     Capture(allowed).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
    /// );
    /// assert_eq!(from_allowed.can_precede((1, 3)), true);
    /// assert_eq!(from_allowed.can_precede((2, 1)), false);
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> Out {
        match &self.closure {
            OneOf2::Variant1(fun) => fun.call(input),
            OneOf2::Variant2(fun) => fun.call(input),
        }
    }

    /// Returns a reference to the captured data.
    pub fn captured_data(&self) -> OneOf2<&C1, &C2> {
        match &self.closure {
            OneOf2::Variant1(x) => OneOf2::Variant1(x.captured_data()),
            OneOf2::Variant2(x) => OneOf2::Variant2(x.captured_data()),
        }
    }

    /// Consumes the closure and returns back the captured data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    /// use std::collections::HashSet;
    ///
    /// type Node = usize; // for brevity
    /// type Edge = (Node, Node); // for brevity
    ///
    /// // captures either () or Vec<HashSet<Node>>
    /// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
    ///
    /// struct Precedence(PrecedenceClosure);
    ///
    /// impl Precedence {
    ///     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var1())
    ///     }
    ///     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var2())
    ///     }
    ///
    ///     fn can_precede(&self, edge: Edge) -> bool {
    ///         self.0.call(edge)
    ///     }
    /// }
    ///
    /// let allowed: Vec<HashSet<Node>> = vec![
    ///     HashSet::from_iter([1, 2, 3].into_iter()),
    ///     HashSet::from_iter([2, 3].into_iter()),
    ///     HashSet::from_iter([3].into_iter()),
    ///     HashSet::from_iter([0].into_iter()),
    /// ];
    /// let from_allowed = Precedence::new_variant2(
    ///     Capture(allowed.clone()).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
    /// );
    /// assert_eq!(from_allowed.can_precede((1, 3)), true);
    /// assert_eq!(from_allowed.can_precede((2, 1)), false);
    ///
    /// let data = from_allowed.0.into_captured_data();
    /// assert!(matches!(data, OneOf2::Variant2(allowed)));
    /// ```
    #[inline(always)]
    pub fn into_captured_data(self) -> OneOf2<C1, C2> {
        match self.closure {
            OneOf2::Variant1(fun) => OneOf2::Variant1(fun.into_captured_data()),
            OneOf2::Variant2(fun) => OneOf2::Variant2(fun.into_captured_data()),
        }
    }

    /// Returns the closure as an `impl Fn(In) -> Out` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    /// use std::collections::HashSet;
    ///
    /// type Node = usize; // for brevity
    /// type Edge = (Node, Node); // for brevity
    ///
    /// // captures either () or Vec<HashSet<Node>>
    /// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
    ///
    /// struct Precedence(PrecedenceClosure);
    ///
    /// impl Precedence {
    ///     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var1())
    ///     }
    ///     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
    ///         Self(closure.into_oneof2_var2())
    ///     }
    ///
    ///     fn get_can_precede(&self) -> impl Fn(Edge) -> bool + '_ {
    ///         self.0.as_fn()
    ///     }
    /// }
    ///
    /// let allow_all = Precedence::new_variant1(Capture(()).fun(|_, _| true));
    /// let fun = allow_all.get_can_precede();
    /// assert_eq!(fun((0, 1)), true);
    /// assert_eq!(fun((10, 21)), true);
    ///
    /// let disallow_all = Precedence::new_variant1(Capture(()).fun(|_, _| false));
    /// let fun = disallow_all.get_can_precede();
    /// assert_eq!(fun((0, 1)), false);
    /// assert_eq!(fun((10, 21)), false);
    ///
    /// let allowed: Vec<HashSet<Node>> = vec![
    ///     HashSet::from_iter([1, 2, 3].into_iter()),
    ///     HashSet::from_iter([2, 3].into_iter()),
    ///     HashSet::from_iter([3].into_iter()),
    ///     HashSet::from_iter([0].into_iter()),
    /// ];
    /// let from_allowed = Precedence::new_variant2(
    ///     Capture(allowed).fun(|allowed, edge| allowed[edge.0].contains(&edge.1)),
    /// );
    /// let fun = from_allowed.get_can_precede();
    /// assert_eq!(fun((1, 3)), true);
    /// assert_eq!(fun((2, 1)), false);
    /// ```
    pub fn as_fn(&self) -> impl Fn(In) -> Out + '_ {
        move |x| self.call(x)
    }
}

impl<Capture, In, Out> Closure<Capture, In, Out> {
    /// Transforms `Closure<C1, In, Out>` into the more general `ClosureOneOf2<C1, C2, In, Out>` for any `C2`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    /// use std::collections::HashSet;
    ///
    /// type Node = usize; // for brevity
    /// type Edge = (Node, Node); // for brevity
    ///
    /// // captures either () or Vec<HashSet<Node>>
    /// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
    ///
    /// struct Precedence(PrecedenceClosure);
    ///
    /// impl Precedence {
    ///     fn new_variant1(closure: Closure<(), Edge, bool>) -> Self {
    ///         // transforms        : Closure<(), Edge, bool>
    ///         // into more general : ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>
    ///         Self(closure.into_oneof2_var1())
    ///     }
    /// }
    /// ```
    pub fn into_oneof2_var1<Var2>(self) -> ClosureOneOf2<Capture, Var2, In, Out> {
        let closure = OneOf2::Variant1(self);
        ClosureOneOf2 { closure }
    }

    /// Transforms `Closure<C2, In, Out>` into the more general `ClosureOneOf2<C1, C2, In, Out>` for any `C1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    /// use std::collections::HashSet;
    ///
    /// type Node = usize; // for brevity
    /// type Edge = (Node, Node); // for brevity
    ///
    /// // captures either () or Vec<HashSet<Node>>
    /// type PrecedenceClosure = ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>;
    ///
    /// struct Precedence(PrecedenceClosure);
    ///
    /// impl Precedence {
    ///     fn new_variant2(closure: Closure<Vec<HashSet<Node>>, Edge, bool>) -> Self {
    ///         // transforms        : Closure<Vec<HashSet<Node>>, Edge, bool>
    ///         // into more general : ClosureOneOf2<(), Vec<HashSet<Node>>, Edge, bool>
    ///         Self(closure.into_oneof2_var2())
    ///     }
    /// }
    /// ```
    pub fn into_oneof2_var2<Var1>(self) -> ClosureOneOf2<Var1, Capture, In, Out> {
        let closure = OneOf2::Variant2(self);
        ClosureOneOf2 { closure }
    }
}

impl<C1, C2, In, Out> Fun<In, Out> for ClosureOneOf2<C1, C2, In, Out> {
    fn call(&self, input: In) -> Out {
        ClosureOneOf2::call(self, input)
    }
}
