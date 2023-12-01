use crate::fun::FunRef;
use std::fmt::Debug;

/// Closure strictly separating the captured data from the function, and hence, having two components:
///
/// * `Capture` is any captured data,
/// * `fn(&Capture, In) -> &Out` is the transformation.
///
/// It represents the transformation `In -> &Out`.
///
/// Note that, unlike trait objects of fn-traits, `Capture` auto-implements `Clone` given that captured data is cloneable.
///
/// **Instead of `ClosureRef`; this closure variant is particularly useful when we capture the data by value and return a reference.**
///
/// # Example
///
/// ```rust
/// use orx_closure::Capture;
///
/// struct Person { name: String }
/// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
/// // name_of_person_with_id: ClosureRef<[Person; 2], usize, str>
/// let name_of_person_with_id =
///     Capture(people).fun_ref(|ppl, id: usize| ppl[id].name.as_str());
///
/// assert_eq!("john", name_of_person_with_id.call(0));
///
/// // alternatively
/// let fun = name_of_person_with_id.as_fn();
/// assert_eq!("doe", fun(1));
/// ```
#[derive(Clone)]
pub struct ClosureRef<Capture, In, Out: ?Sized> {
    capture: Capture,
    fun: fn(&Capture, In) -> &Out,
}

impl<Capture: Debug, In, Out: ?Sized> Debug for ClosureRef<Capture, In, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosureRef")
            .field("capture", &self.capture)
            .finish()
    }
}

impl<Capture, In, Out: ?Sized> ClosureRef<Capture, In, Out> {
    pub(super) fn new(capture: Capture, fun: fn(&Capture, In) -> &Out) -> Self {
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
    /// // name_of_person_with_id: ClosureRef<[Person; 2], usize, str>
    /// let name_of_person_with_id =
    ///     Capture(people).fun_ref(|ppl, id: usize| ppl[id].name.as_str());
    ///
    /// assert_eq!("john", name_of_person_with_id.call(0));
    /// ```
    #[inline(always)]
    pub fn call(&self, input: In) -> &Out {
        (self.fun)(&self.capture, input)
    }

    /// Returns a reference to the captured data.
    #[inline(always)]
    pub fn captured_data(&self) -> &Capture {
        &self.capture
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
    /// let get_number = Capture(data).fun_ref(|data, i| &data.0[i]);
    ///
    /// assert_eq!(&10, get_number.call(0));
    /// assert_eq!(&12, get_number.call(2));
    ///
    /// let _data: ExpensiveData = get_number.into_captured_data();
    /// ```
    pub fn into_captured_data(self) -> Capture {
        self.capture
    }

    /// Returns the closure as an `impl Fn(In) -> &Out` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// struct Person { name: String }
    /// let people = [Person { name: "john".to_string() }, Person { name: "doe".to_string() }];
    /// // name_of_person_with_id: ClosureRef<[Person; 2], usize, str>
    /// let name_of_person_with_id =
    ///     Capture(people).fun_ref(|ppl, id: usize| ppl[id].name.as_str());
    ///
    /// let fun = name_of_person_with_id.as_fn();
    /// assert_eq!("doe", fun(1));
    /// ```
    pub fn as_fn<'a>(&'a self) -> impl Fn(In) -> &'a Out {
        move |x| self.call(x)
    }
}

impl<Capture, In, Out: ?Sized> FunRef<In, Out> for ClosureRef<Capture, In, Out> {
    fn call(&self, input: In) -> &Out {
        ClosureRef::call(self, input)
    }
}
