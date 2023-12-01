use crate::{fun::FunResRef, ClosureResRef, OneOf2};

type UnionClosure<C1, C2, In, Out, Error> =
    OneOf2<ClosureResRef<C1, In, Out, Error>, ClosureResRef<C2, In, Out, Error>>;

/// `ClosureResRefOneOf2<C1, C2, In, Out, Error>` is a union of two closures:
///
/// * `ClosureResRef<C1, In, Out, Error>`
/// * `ClosureResRef<C2, In, Out, Error>`
///
/// This is useful when it is possible that the closure might capture and work with either of the two types of data `C1` and `C2`.
///
/// It represents the transformation `In -> Option<&Out>`.
///
/// Note that, unlike trait objects of fn-traits, `ClosureResRefOneOf2` auto-implements `Clone` given that captured data variants are cloneable.
///
/// **Instead of `ClosureOneOf2`; this closure variant is particularly useful when we capture the data by value and return a result of a reference.**
///
/// # Example
///
/// ```rust
/// use orx_closure::*;
///
/// type Toy = String;
/// type MyErr = &'static str;
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
///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
/// }
///
/// // cats
/// let cats = vec![Cat {
///     name: "bella".to_string(),
///     favorite_toys: vec!["ball".to_string()],
/// }];
/// let present_ideas = PresentIdeas {
///     for_pet: Capture(cats)
///         .fun_result_ref(|cats, name| {
///             cats.iter()
///                 .find(|cat| cat.name == name)
///                 .map(|cat| cat.favorite_toys.as_slice())
///                 .ok_or("pet name is absent")
///         })
///         .into_oneof2_var1(),
/// };
///
/// assert_eq!(
///     Ok(vec!["ball".to_string()].as_slice()),
///     present_ideas.for_pet.call("bella")
/// );
/// assert_eq!(
///     Err("pet name is absent"),
///     present_ideas.for_pet.call("luna")
/// );
///
/// // dogs
/// let dogs = vec![Dog {
///     name: "luke".to_string(),
///     nickname: "dogzilla".to_string(),
///     favorite_toys: vec!["toy turtle".to_string()],
/// }];
/// let present_ideas = PresentIdeas {
///     for_pet: Capture(dogs)
///         .fun_result_ref(|dogs, name| {
///             dogs.iter()
///                 .find(|dog| dog.name == name || dog.nickname == name)
///                 .map(|dog| dog.favorite_toys.as_slice())
///                 .ok_or("pet name is absent")
///         })
///         .into_oneof2_var2(),
/// };
/// assert_eq!(
///     Ok(vec!["toy turtle".to_string()].as_slice()),
///     present_ideas.for_pet.call("luke")
/// );
/// assert_eq!(
///     Ok(vec!["toy turtle".to_string()].as_slice()),
///     present_ideas.for_pet.call("dogzilla")
/// );
/// assert_eq!(Err("pet name is absent"), present_ideas.for_pet.call("tux"));
/// ```
#[derive(Clone, Debug)]
pub struct ClosureResRefOneOf2<C1, C2, In, Out: ?Sized, Error> {
    closure: UnionClosure<C1, C2, In, Out, Error>,
}
impl<C1, C2, In, Out: ?Sized, Error> ClosureResRefOneOf2<C1, C2, In, Out, Error> {
    /// Calls the closure with the given `input`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// type MyErr = &'static str;
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
    ///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_result_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .ok_or("pet name is absent")
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// // calling the closure with different pat names
    /// assert_eq!(
    ///     Ok(vec!["ball".to_string()].as_slice()),
    ///     present_ideas.for_pet.call("bella")
    /// );
    /// assert_eq!(
    ///     Err("pet name is absent"),
    ///     present_ideas.for_pet.call("luna")
    /// );
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> Result<&Out, Error> {
        match &self.closure {
            OneOf2::Variant1(fun) => fun.call(input),
            OneOf2::Variant2(fun) => fun.call(input),
        }
    }

    /// Returns a reference to the captured data.
    #[inline(always)]
    pub fn captured_data(&self) -> OneOf2<&C1, &C2> {
        match &self.closure {
            OneOf2::Variant1(x) => OneOf2::Variant1(x.captured_data()),
            OneOf2::Variant2(x) => OneOf2::Variant2(x.captured_data()),
        }
    }

    /// Consumes the closure and returns back the captured data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// type MyErr = &'static str;
    ///
    /// #[derive(Debug, Clone, PartialEq, Eq)]
    /// struct Cat {
    ///     name: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// #[derive(Debug, Clone, PartialEq, Eq)]
    /// struct Dog {
    ///     name: String,
    ///     nickname: String,
    ///     favorite_toys: Vec<Toy>,
    /// }
    ///
    /// struct PresentIdeas<'a> {
    ///     // for cats or dogs
    ///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats.clone())
    ///         .fun_result_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .ok_or("pet name is absent")
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// // calling the closure with different pat names
    /// assert_eq!(
    ///     Ok(vec!["ball".to_string()].as_slice()),
    ///     present_ideas.for_pet.call("bella")
    /// );
    /// assert_eq!(
    ///     Err("pet name is absent"),
    ///     present_ideas.for_pet.call("luna")
    /// );
    ///
    /// // get back the captured data which can be one of the two options: cats or dogs:
    ///
    /// let data = present_ideas.for_pet.into_captured_data();
    ///
    /// assert_eq!(data, OneOf2::Variant1(cats));
    /// ```
    pub fn into_captured_data(self) -> OneOf2<C1, C2> {
        match self.closure {
            OneOf2::Variant1(fun) => OneOf2::Variant1(fun.into_captured_data()),
            OneOf2::Variant2(fun) => OneOf2::Variant2(fun.into_captured_data()),
        }
    }

    /// Returns the closure as an `impl Fn(In) -> Result<&Out>` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// type MyErr = &'static str;
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
    ///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_result_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .ok_or("pet name is absent")
    ///         })
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// // function accepting an instance of the `Fn(&str) -> &[Toy]` trait
    /// fn create_presents<'a, F: Fn(&'a str) -> Result<&'a [Toy], MyErr>>(present_ideas_for: F) -> Vec<Toy> {
    ///     ["bella", "luna"]
    ///         .iter()
    ///         .flat_map(|name| present_ideas_for(name).unwrap_or(&[]).iter().cloned())
    ///         .collect()
    /// }
    ///
    /// // we can conveniently create the `Fn` with `as_fn`
    /// let presents = create_presents(present_ideas.for_pet.as_fn());
    /// assert_eq!(&["ball".to_string()], presents.as_slice());
    /// ```
    pub fn as_fn<'a>(&'a self) -> impl Fn(In) -> Result<&'a Out, Error> {
        move |x| self.call(x)
    }
}

impl<Capture, In, Out: ?Sized, Error> ClosureResRef<Capture, In, Out, Error> {
    /// Transforms `ClosureResRef<C1, In, Out>` into the more general `ClosureResRefOneOf2<C1, C2, In, Out>` for any `C2`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// type MyErr = &'static str;
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
    ///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    /// }
    ///
    /// // cats
    /// let cats = vec![Cat {
    ///     name: "bella".to_string(),
    ///     favorite_toys: vec!["ball".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(cats)
    ///         .fun_result_ref(|cats, name| {
    ///             cats.iter()
    ///                 .find(|cat| cat.name == name)
    ///                 .map(|cat| cat.favorite_toys.as_slice())
    ///                 .ok_or("pet name is absent")
    ///         })
    ///         // transforms        : ClosureResRef<Vec<Cat>, &str, [Toy], MyErr>
    ///         // into more general : ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy], MyErr>
    ///         .into_oneof2_var1(),
    /// };
    ///
    /// assert_eq!(
    ///     Ok(vec!["ball".to_string()].as_slice()),
    ///     present_ideas.for_pet.call("bella")
    /// );
    /// assert_eq!(
    ///     Err("pet name is absent"),
    ///     present_ideas.for_pet.call("luna")
    /// );
    /// ```
    pub fn into_oneof2_var1<Var2>(self) -> ClosureResRefOneOf2<Capture, Var2, In, Out, Error> {
        let closure = OneOf2::Variant1(self);
        ClosureResRefOneOf2 { closure }
    }

    /// Transforms `ClosureOptRef<C2, In, Out>` into the more general `ClosureRefOneOf2<C1, C2, In, Out>` for any `C1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// type Toy = String;
    /// type MyErr = &'static str;
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
    ///     for_pet: ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &'a str, [Toy], MyErr>,
    /// }
    ///
    /// let dogs = vec![Dog {
    ///     name: "luke".to_string(),
    ///     nickname: "dogzilla".to_string(),
    ///     favorite_toys: vec!["toy turtle".to_string()],
    /// }];
    /// let present_ideas = PresentIdeas {
    ///     for_pet: Capture(dogs)
    ///         .fun_result_ref(|dogs, name| {
    ///             dogs.iter()
    ///                 .find(|dog| dog.name == name || dog.nickname == name)
    ///                 .map(|dog| dog.favorite_toys.as_slice())
    ///                 .ok_or("pet name is absent")
    ///         })
    ///         // transforms        : ClosureResRef<Vec<Dog>, &str, [Toy], MyErr>
    ///         // into more general : ClosureResRefOneOf2<Vec<Cat>, Vec<Dog>, &str, [Toy], MyErr>
    ///         .into_oneof2_var2(),
    /// };
    /// assert_eq!(
    ///     Ok(vec!["toy turtle".to_string()].as_slice()),
    ///     present_ideas.for_pet.call("luke")
    /// );
    /// assert_eq!(
    ///     Ok(vec!["toy turtle".to_string()].as_slice()),
    ///     present_ideas.for_pet.call("dogzilla")
    /// );
    /// assert_eq!(Err("pet name is absent"), present_ideas.for_pet.call("tux"));
    /// ```
    pub fn into_oneof2_var2<Var1>(self) -> ClosureResRefOneOf2<Var1, Capture, In, Out, Error> {
        let closure = OneOf2::Variant2(self);
        ClosureResRefOneOf2 { closure }
    }
}

impl<C1, C2, In, Out: ?Sized, Error> FunResRef<In, Out, Error>
    for ClosureResRefOneOf2<C1, C2, In, Out, Error>
{
    fn call(&self, input: In) -> Result<&Out, Error> {
        ClosureResRefOneOf2::call(self, input)
    }
}
