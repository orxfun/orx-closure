use crate::{fun::Fun, Closure, OneOf4};

type UnionClosures<C1, C2, C3, C4, In, Out> =
    OneOf4<Closure<C1, In, Out>, Closure<C2, In, Out>, Closure<C3, In, Out>, Closure<C4, In, Out>>;

/// `ClosureOneOf4<C1, C2, C3, C4, In, Out>` is a union of three closures:
///
/// * `Closure<C1, In, Out>`
/// * `Closure<C2, In, Out>`
/// * `Closure<C3, In, Out>`
/// * `Closure<C4, In, Out>`
///
/// This is useful when it is possible that the closure might capture and work with either of the three types of data `C1`, `C2`, `C3` and `C4`.
///
/// It represents the transformation `In -> Out`.
///
/// Note that, unlike trait objects of fn-traits, `ClosureOneOf4` auto-implements `Clone` given that captured data variants are cloneable.
///
/// # Example
///
/// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
pub struct ClosureOneOf4<C1, C2, C3, C4, In, Out> {
    closure: UnionClosures<C1, C2, C3, C4, In, Out>,
}
impl<C1, C2, C3, C4, In, Out> ClosureOneOf4<C1, C2, C3, C4, In, Out> {
    /// Calls the closure with the given `input`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
            OneOf4::Variant1(fun) => fun.call(input),
            OneOf4::Variant2(fun) => fun.call(input),
            OneOf4::Variant3(fun) => fun.call(input),
            OneOf4::Variant4(fun) => fun.call(input),
        }
    }

    /// Returns a reference to the captured data.
    pub fn captured_data(&self) -> OneOf4<&C1, &C2, &C3, &C4> {
        match &self.closure {
            OneOf4::Variant1(x) => OneOf4::Variant1(x.captured_data()),
            OneOf4::Variant2(x) => OneOf4::Variant2(x.captured_data()),
            OneOf4::Variant3(x) => OneOf4::Variant3(x.captured_data()),
            OneOf4::Variant4(x) => OneOf4::Variant4(x.captured_data()),
        }
    }

    /// Consumes the closure and returns back the captured data.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
    pub fn into_captured_data(self) -> OneOf4<C1, C2, C3, C4> {
        match self.closure {
            OneOf4::Variant1(fun) => OneOf4::Variant1(fun.into_captured_data()),
            OneOf4::Variant2(fun) => OneOf4::Variant2(fun.into_captured_data()),
            OneOf4::Variant3(fun) => OneOf4::Variant3(fun.into_captured_data()),
            OneOf4::Variant4(fun) => OneOf4::Variant4(fun.into_captured_data()),
        }
    }

    /// Returns the closure as an `impl Fn(In) -> Out` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
    /// Transforms `Closure<C1, In, Out>` into the more general `ClosureOneOf4<C1, C2, C3, C4, In, Out>` for any `C2`, `C3` and `C4`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf3 is only a generalization of the below for three different capture types.*
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
    pub fn into_oneof4_var1<Var2, Var3, Var4>(
        self,
    ) -> ClosureOneOf4<Capture, Var2, Var3, Var4, In, Out> {
        let closure = OneOf4::Variant1(self);
        ClosureOneOf4 { closure }
    }

    /// Transforms `Closure<C2, In, Out>` into the more general `ClosureOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C3` and `C4`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
    pub fn into_oneof4_var2<Var1, Var3, Var4>(
        self,
    ) -> ClosureOneOf4<Var1, Capture, Var3, Var4, In, Out> {
        let closure = OneOf4::Variant2(self);
        ClosureOneOf4 { closure }
    }

    /// Transforms `Closure<C3, In, Out>` into the more general `ClosureOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C2` and `C4`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
    pub fn into_oneof4_var3<Var1, Var2, Var4>(
        self,
    ) -> ClosureOneOf4<Var1, Var2, Capture, Var4, In, Out> {
        let closure = OneOf4::Variant3(self);
        ClosureOneOf4 { closure }
    }

    /// Transforms `Closure<C4, In, Out>` into the more general `ClosureOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C2` and `C3`.
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureOneOf4 is only a generalization of the below for three different capture types.*
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
    pub fn into_oneof4_var4<Var1, Var2, Var3>(
        self,
    ) -> ClosureOneOf4<Var1, Var2, Var3, Capture, In, Out> {
        let closure = OneOf4::Variant4(self);
        ClosureOneOf4 { closure }
    }
}

impl<C1, C2, C3, C4, In, Out> Fun<In, Out> for ClosureOneOf4<C1, C2, C3, C4, In, Out> {
    fn call(&self, input: In) -> Out {
        ClosureOneOf4::call(self, input)
    }
}
