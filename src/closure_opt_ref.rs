use std::fmt::Debug;

/// Closure strictly separating the captured data from the function, and hence, having two components:
///
/// * `Capture` is any captured data,
/// * `fn(&Capture, In) -> Option<&Out>` is the transformation.
///
/// It represents the transformation `In -> Option<&Out>`.
///
/// Note that, unlike trait objects of fn-traits, `Capture` auto-implements `Clone` given that captured data is cloneable.
///
/// **Instead of `ClosureOptRef`; this closure variant is particularly useful when we capture the data by value and return an `Option` of a reference.**
///
/// # Example
///
/// ```rust
/// use orx_closure::Capture;
///
/// struct Person { name: String }
/// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
/// // name_of_person_with_id: ClosureOptRef<[Person; 2], usize, str>
/// let name_of_person_with_id =
///     Capture(people).fun_option_ref(|ppl, id: usize| ppl.get(id).map(|p| p.name.as_str()));
///
/// assert_eq!(Some("john"), name_of_person_with_id.call(0));
/// assert_eq!(None, name_of_person_with_id.call(42));
///
/// // alternatively
/// let fun = name_of_person_with_id.as_fn();
/// assert_eq!(Some("doe"), fun(1));
/// ```
#[derive(Clone)]
pub struct ClosureOptRef<Capture, In, Out: ?Sized> {
    capture: Capture,
    fun: fn(&Capture, In) -> Option<&Out>,
}

impl<Capture: Debug, In, Out: ?Sized> Debug for ClosureOptRef<Capture, In, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosureOptRef")
            .field("capture", &self.capture)
            .finish()
    }
}

impl<Capture, In, Out: ?Sized> ClosureOptRef<Capture, In, Out> {
    pub(super) fn new(capture: Capture, fun: fn(&Capture, In) -> Option<&Out>) -> Self {
        Self { capture, fun }
    }

    /// Calls the closure with the given `input`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// struct Person { name: String }
    /// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
    /// // name_of_person_with_id: ClosureOptRef<[Person; 2], usize, str>
    /// let name_of_person_with_id =
    ///     Capture(people).fun_option_ref(|ppl, id: usize| ppl.get(id).map(|p| p.name.as_str()));
    ///
    /// assert_eq!(Some("john"), name_of_person_with_id.call(0));
    /// assert_eq!(None, name_of_person_with_id.call(42));
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> Option<&Out> {
        (self.fun)(&self.capture, input)
    }

    /// Consumes the closure and returns back the captured data.
    ///
    /// # Example
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// struct ExpensiveData(Vec<i32>);
    ///
    /// let data = ExpensiveData(vec![10, 11, 12]);
    ///
    /// let get_number = Capture(data).fun_option_ref(|data, i| data.0.get(i));
    ///
    /// assert_eq!(Some(&10), get_number.call(0));
    /// assert_eq!(Some(&12), get_number.call(2));
    ///
    /// let _data: ExpensiveData = get_number.into_captured_data();
    /// ```
    pub fn into_captured_data(self) -> Capture {
        self.capture
    }

    /// Returns the closure as an `impl Fn(In) -> Option<&Out>` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// struct Person { name: String }
    /// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
    /// // name_of_person_with_id: ClosureOptRef<[Person; 2], usize, str>
    /// let name_of_person_with_id =
    ///     Capture(people).fun_option_ref(|ppl, id: usize| ppl.get(id).map(|p| p.name.as_str()));
    ///
    /// // alternatively
    /// let fun = name_of_person_with_id.as_fn();
    /// assert_eq!(Some("doe"), fun(1));
    /// ```
    pub fn as_fn<'a>(&'a self) -> impl Fn(In) -> Option<&'a Out> {
        move |x| self.call(x)
    }
}
