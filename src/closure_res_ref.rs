use std::fmt::Debug;

/// Closure strictly separating the captured data from the function, and hence, having two components:
///
/// * `Capture` is any captured data,
/// * `fn(&Capture, In) -> Result<&Out, Error>` is the transformation.
///
/// It represents the transformation `In -> Result<&Out, Error>`.
///
/// Note that, unlike trait objects of fn-traits, `Capture` auto-implements `Clone` given that captured data is cloneable.
///
/// **Instead of `ClosureResRef`; this closure variant is particularly useful when we capture the data by value and return an `Result`` where the Ok variant is a reference.**
///
/// # Example
///
/// ```rust
/// use orx_closure::Capture;
///
/// struct Person { name: String }
/// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
/// // name_of_person_with_id: ClosureResRef<[Person; 2], usize, str, String>
/// let name_of_person_with_id = Capture(people).fun_result_ref(|ppl, id: usize| {
///     ppl.get(id)
///         .map(|p| p.name.as_str())
///         .ok_or_else(|| "unknown id".to_string())
/// });
///
/// assert_eq!(Ok("john"), name_of_person_with_id.call(0));
/// assert_eq!(Err("unknown id".to_string()), name_of_person_with_id.call(42));
///
/// // alternatively
/// let fun = name_of_person_with_id.as_fn();
/// assert_eq!(Ok("doe"), fun(1));
/// ```
#[derive(Clone)]
pub struct ClosureResRef<Capture, In, Out: ?Sized, Error> {
    capture: Capture,
    fun: fn(&Capture, In) -> Result<&Out, Error>,
}

impl<Capture: Debug, In, Out: ?Sized, Error> Debug for ClosureResRef<Capture, In, Out, Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosureResRef")
            .field("capture", &self.capture)
            .finish()
    }
}

impl<Capture, In, Out: ?Sized, Error> ClosureResRef<Capture, In, Out, Error> {
    pub(super) fn new(capture: Capture, fun: fn(&Capture, In) -> Result<&Out, Error>) -> Self {
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
    /// // name_of_person_with_id: ClosureResRef<[Person; 2], usize, str, String>
    /// let name_of_person_with_id = Capture(people).fun_result_ref(|ppl, id: usize| {
    ///     ppl.get(id)
    ///         .map(|p| p.name.as_str())
    ///         .ok_or_else(|| "unknown id".to_string())
    /// });
    ///
    /// assert_eq!(Ok("john"), name_of_person_with_id.call(0));
    /// assert_eq!(Err("unknown id".to_string()), name_of_person_with_id.call(42));
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> Result<&Out, Error> {
        (self.fun)(&self.capture, input)
    }

    /// Consumes the closure and returns back the captured data.
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// struct ExpensiveData(Vec<i32>);
    ///
    /// let data = ExpensiveData(vec![10, 11, 12]);
    ///
    /// let get_number = Capture(data).fun_result_ref(|data, i| data.0.get(i).ok_or("!!"));
    ///
    /// assert_eq!(Ok(&10), get_number.call(0));
    /// assert_eq!(Ok(&12), get_number.call(2));
    ///
    /// let _data: ExpensiveData = get_number.into_captured_data();
    /// ```
    pub fn into_captured_data(self) -> Capture {
        self.capture
    }

    /// Returns the closure as an `impl Fn(In) -> Result<&Out, String>` struct, allowing the convenience
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
    /// // name_of_person_with_id: ClosureResRef<[Person; 2], usize, str, String>
    /// let name_of_person_with_id = Capture(people).fun_result_ref(|ppl, id: usize| {
    ///     ppl.get(id)
    ///         .map(|p| p.name.as_str())
    ///         .ok_or_else(|| "unknown id".to_string())
    /// });
    ///
    /// let fun = name_of_person_with_id.as_fn();
    /// assert_eq!(Ok("doe"), fun(1));
    /// ```
    pub fn as_fn<'a>(&'a self) -> impl Fn(In) -> Result<&'a Out, Error> {
        move |x| self.call(x)
    }
}
