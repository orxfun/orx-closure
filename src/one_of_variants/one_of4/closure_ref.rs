use crate::{fun::FunRef, ClosureRef, OneOf4};

type UnionClosures<C1, C2, C3, C4, In, Out> = OneOf4<
    ClosureRef<C1, In, Out>,
    ClosureRef<C2, In, Out>,
    ClosureRef<C3, In, Out>,
    ClosureRef<C4, In, Out>,
>;

/// `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>` is a union of three closures:
///
/// * `ClosureRef<C1, In, Out>`
/// * `ClosureRef<C2, In, Out>`
/// * `ClosureRef<C3, In, Out>`
/// * `ClosureRef<C4, In, Out>`
///
/// This is useful when it is possible that the closure might capture and work with either of the three types of data `C1`, `C2`, `C3` and `C4`.
///
/// It represents the transformation `In -> &Out`.
///
/// Note that, unlike trait objects of fn-traits, `ClosureRefOneOf3` auto-implements `Clone` given that captured data variants are cloneable.
///
/// **Instead of `ClosureOneOf4`; this closure variant is particularly useful when we capture the data by value and return a reference.**
///
/// # Example
///
/// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
///
/// ```rust
/// use orx_closure::*;
///
/// type Toy = String;
/// struct Cat {
///     name: String,
///     favorite_toys: Vec<Toy>,
/// }
/// struct Dog {
///     name: String,
///     nickname: String,
///     favorite_toys: Vec<Toy>,
/// }
///
/// struct PresentIdeas<'a> {
///     // for cats or dogs
///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
/// }
///
/// // cats
/// let cats = vec![Cat {
///     name: "bella".to_string(),
///     favorite_toys: vec!["ball".to_string()],
/// }];
/// let present_ideas = PresentIdeas {
///     for_pet: Capture(cats)
///         .fun_ref(|cats, name| {
///             cats.iter()
///                 .find(|cat| cat.name == name)
///                 .map(|cat| cat.favorite_toys.as_slice())
///                 .unwrap_or(&[])
///         })
///         .into_oneof2_var1(),
/// };
///
/// assert_eq!(&["ball".to_string()], present_ideas.for_pet.call("bella"));
/// assert!(present_ideas.for_pet.call("luna").is_empty());
///
/// // dogs
/// let dogs = vec![Dog {
///     name: "luke".to_string(),
///     nickname: "dogzilla".to_string(),
///     favorite_toys: vec!["toy turtle".to_string()],
/// }];
/// let present_ideas = PresentIdeas {
///     for_pet: Capture(dogs)
///         .fun_ref(|dogs, name| {
///             dogs.iter()
///                 .find(|dog| dog.name == name || dog.nickname == name)
///                 .map(|dog| dog.favorite_toys.as_slice())
///                 .unwrap_or(&[])
///         })
///         .into_oneof2_var2(),
/// };
/// assert_eq!(
///     &["toy turtle".to_string()],
///     present_ideas.for_pet.call("luke")
/// );
/// assert_eq!(
///     &["toy turtle".to_string()],
///     present_ideas.for_pet.call("dogzilla")
/// );
/// assert!(present_ideas.for_pet.call("tux").is_empty());
/// ```
#[derive(Clone, Debug)]
pub struct ClosureRefOneOf4<C1, C2, C3, C4, In, Out: ?Sized> {
    closure: UnionClosures<C1, C2, C3, C4, In, Out>,
}
impl<C1, C2, C3, C4, In, Out: ?Sized> ClosureRefOneOf4<C1, C2, C3, C4, In, Out> {
    /// Calls the closure with the given `input`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// // calling the closure
    /// assert_eq!(&["ball".to_string()], present_ideas.for_pet.call("bella"));
    /// assert!(present_ideas.for_pet.call("luna").is_empty());
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> &Out {
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
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// #[derive(Debug, Clone, PartialEq, Eq)]
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// #[derive(Debug, Clone, PartialEq, Eq)]
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats.clone()) // clone to use `cats` for validation later
    ///         .fun_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// assert_eq!(&["ball".to_string()], present_ideas.for_pet.call("bella"));
    /// assert!(present_ideas.for_pet.call("luna").is_empty());
    ///
    /// // get back the captured data which can be one of the two options: cats or dogs:
    ///
    /// let data = present_ideas.for_pet.into_captured_data();
    ///
    /// assert_eq!(data, OneOf2::Variant1(cats));
    /// ```
    #[inline(always)]
    pub fn into_captured_data(self) -> OneOf4<C1, C2, C3, C4> {
        match self.closure {
            OneOf4::Variant1(fun) => OneOf4::Variant1(fun.into_captured_data()),
            OneOf4::Variant2(fun) => OneOf4::Variant2(fun.into_captured_data()),
            OneOf4::Variant3(fun) => OneOf4::Variant3(fun.into_captured_data()),
            OneOf4::Variant4(fun) => OneOf4::Variant4(fun.into_captured_data()),
        }
    }

    /// Returns the closure as an `impl Fn(In) -> &Out` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog;
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// // function accepting an instance of the `Fn(&str) -> &[Toy]` trait
    /// fn create_presents<'a, F: Fn(&'a str) -> &'a [Toy]>(present_ideas_for: F) -> Vec<Toy> {
    ///     ["bella", "luna"]
    ///         .iter()
    ///         .flat_map(|name| present_ideas_for(name).iter().cloned())
    ///         .collect()
    /// }
    ///
    /// // we can conveniently create the `Fn` with `as_fn`
    /// let presents = create_presents(present_ideas.for_pet.as_fn());
    /// assert_eq!(&["ball".to_string()], presents.as_slice());
    /// ```
    pub fn as_fn<'a>(&'a self) -> impl Fn(In) -> &'a Out {
        move |x| self.call(x)
    }
}

impl<Capture, In, Out: ?Sized> ClosureRef<Capture, In, Out> {
    /// Transforms `ClosureRef<C1, In, Out>` into the more general `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>` for any `C2`, `C3` and `C4`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         // transforms        : ClosureRef<Vec<Cat>, &str, [Toy]>
    ///         // into more general : ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy]>
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// assert_eq!(&["ball".to_string()], present_ideas.for_pet.call("bella"));
    /// assert!(present_ideas.for_pet.call("luna").is_empty());
    /// ```
    pub fn into_oneof4_var1<Var2, Var3, Var4>(
        self,
    ) -> ClosureRefOneOf4<Capture, Var2, Var3, Var4, In, Out> {
        let closure = OneOf4::Variant1(self);
        ClosureRefOneOf4 { closure }
    }

    /// Transforms `ClosureRef<C2, In, Out>` into the more general `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C3` and `C4`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// let dogs = vec![Dog {
    ///     name: "luke".to_string(),
    ///     nickname: "dogzilla".to_string(),
    ///     favorite_toys: vec!["toy turtle".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(dogs)
    ///         .fun_ref(|dogs, name| {
    ///             dogs.iter()
    ///                 .find(|dog| dog.name == name || dog.nickname == name)
    ///                 .map(|dog| dog.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         // transforms        : ClosureRef<Vec<Dog>, &str, [Toy]>
    ///         // into more general : ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy]>
    ///         .into_oneof2_var2(),
    /// };
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("luke")
    /// );
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("dogzilla")
    /// );
    /// assert!(present_ideas.for_pet.call("tux").is_empty());
    /// ```
    pub fn into_oneof4_var2<Var1, Var3, Var4>(
        self,
    ) -> ClosureRefOneOf4<Var1, Capture, Var3, Var4, In, Out> {
        let closure = OneOf4::Variant2(self);
        ClosureRefOneOf4 { closure }
    }

    /// Transforms `ClosureRef<C3, In, Out>` into the more general `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C2` and `C4`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// let dogs = vec![Dog {
    ///     name: "luke".to_string(),
    ///     nickname: "dogzilla".to_string(),
    ///     favorite_toys: vec!["toy turtle".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(dogs)
    ///         .fun_ref(|dogs, name| {
    ///             dogs.iter()
    ///                 .find(|dog| dog.name == name || dog.nickname == name)
    ///                 .map(|dog| dog.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         // transforms        : ClosureRef<Vec<Dog>, &str, [Toy]>
    ///         // into more general : ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy]>
    ///         .into_oneof2_var2(),
    /// };
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("luke")
    /// );
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("dogzilla")
    /// );
    /// assert!(present_ideas.for_pet.call("tux").is_empty());
    /// ```
    pub fn into_oneof4_var3<Var1, Var2, Var4>(
        self,
    ) -> ClosureRefOneOf4<Var1, Var2, Capture, Var4, In, Out> {
        let closure = OneOf4::Variant3(self);
        ClosureRefOneOf4 { closure }
    }

    /// Transforms `ClosureRef<C4, In, Out>` into the more general `ClosureRefOneOf4<C1, C2, C3, C4, In, Out>` for any `C1`, `C2` and `C3`.
    ///
    /// # Example
    ///
    /// *The example below illustrates the usage of the closure over two possible types of captures; however, ClosureRefOneOf4 is only a generalization of the below for three different capture types.*
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy]>,
    /// }
    ///
    /// let dogs = vec![Dog {
    ///     name: "luke".to_string(),
    ///     nickname: "dogzilla".to_string(),
    ///     favorite_toys: vec!["toy turtle".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(dogs)
    ///         .fun_ref(|dogs, name| {
    ///             dogs.iter()
    ///                 .find(|dog| dog.name == name || dog.nickname == name)
    ///                 .map(|dog| dog.favorite_toys.as_slice())
    ///                 .unwrap_or(&[])
    ///         })
    ///         // transforms        : ClosureRef<Vec<Dog>, &str, [Toy]>
    ///         // into more general : ClosureRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy]>
    ///         .into_oneof2_var2(),
    /// };
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("luke")
    /// );
    /// assert_eq!(
    ///     &["toy turtle".to_string()],
    ///     present_ideas.for_pet.call("dogzilla")
    /// );
    /// assert!(present_ideas.for_pet.call("tux").is_empty());
    /// ```
    pub fn into_oneof4_var4<Var1, Var2, Var3>(
        self,
    ) -> ClosureRefOneOf4<Var1, Var2, Var3, Capture, In, Out> {
        let closure = OneOf4::Variant4(self);
        ClosureRefOneOf4 { closure }
    }
}

impl<C1, C2, C3, C4, In, Out: ?Sized> FunRef<In, Out>
    for ClosureRefOneOf4<C1, C2, C3, C4, In, Out>
{
    fn call(&self, input: In) -> &Out {
        ClosureRefOneOf4::call(self, input)
    }
}
