use crate::{closure_ref::ClosureRef, closure_val::Closure, ClosureOptRef, ClosureResRef};

/// A utility wrapper which simply wraps around data to be captured and allows methods to define desired closures.
///
/// `Closure`s defined in this crate are built by
///
/// * `Capture(data)` which does nothing but captures the `data`,
/// * followed by:
///   * `fun(fn)` to create a `Closure`
///   * `fun_ref(fn)` to create a `ClosureRef`
///   * `fun_option_ref(fn)` to create a `ClosureOptRef`
///   * `fun_result_ref(fn)` to create a `ClosureResRef`
///
/// where `fn` is a non-capturing anonymous function of the correct signature.
///
/// All these methods are consuming moving the data into the corresponding closure.
///
/// The desired way to capture the data is decided by the caller:
///
/// * `Capture(data)` moves the data into the capture, and eventually to the closure,
/// * `Capture(&data)` only captures a reference to the `data`,
/// * `Capture(data.clone())` moves a clone of the `data`, etc.
///
/// **Important note related to how we capture and closure variants:**
///
/// * `Closure` created by the `fun` is sufficient for all types:
///   * when we capture by reference, and/or
///   * when we return a value, rather than a reference.
///
/// * To deal with lifetime errors, we need:
///   * `ClosureRef` created by `fun_ref` when we capture the ownership of the data and return a reference,
///   * `ClosureOptRef` created by `fun_option_ref` when we capture the ownership of the data and return an `Option` of a reference,
///   * `ClosureResRef` created by `fun_result_ref` when we capture the ownership of the data and return a `Result` of a reference.
///
/// # Examples
///
/// ```rust
/// use orx_closure::*;
/// use std::rc::Rc;
///
/// let numbers = vec![42];
///
/// // fun is sufficient in all cases when the data is captured by reference
/// let fun = Capture(&numbers).fun(|vec, i| vec[i]);
/// assert_eq!(42, fun.call(0));
///
/// let fun = Capture(&numbers).fun(|vec, i| &vec[i]);
/// assert_eq!(&42, fun.call(0));
///
/// let fun = Capture(&numbers).fun(|vec, i| vec.get(i));
/// assert_eq!(Some(&42), fun.call(0));
///
/// let fun = Capture(&numbers).fun(|vec, i| vec.get(i).ok_or("no-data"));
/// assert_eq!(Ok(&42), fun.call(0));
///
/// // we need the other variants
/// let fun = Capture(numbers.clone()).fun_ref(|vec, i| &vec[i]);
/// assert_eq!(&42, fun.call(0));
///
/// let fun = Capture(Rc::new(numbers.clone())).fun_option_ref(|vec, i| vec.get(i));
/// assert_eq!(Some(&42), fun.call(0));
///
/// let fun = Capture(numbers).fun_result_ref(|vec, i| vec.get(i).ok_or("no-data"));
/// assert_eq!(Ok(&42), fun.call(0));
/// ```
pub struct Capture<Data>(pub Data);

impl<Data> Capture<Data> {
    /// Defines a `Closure<Data, In, Out>` capturing `Data` and defining `In -> Out` transformation.
    ///
    /// Consumes the `Capture` and moves the captured data inside the created closure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// let base = 2;
    /// let modulo = Capture(base).fun(|b, n| n % b);
    ///
    /// assert_eq!(0, modulo.call(42));
    /// assert_eq!(1, modulo.call(7));
    /// ```
    pub fn fun<In, Out>(self, fun: fn(&Data, In) -> Out) -> Closure<Data, In, Out> {
        Closure::new(self.0, fun)
    }

    /// Defines a `ClosureRef<Data, In, Out>` capturing `Data` and defining `In -> &Out` transformation.
    ///
    /// Consumes the `Capture` and moves the captured data inside the created closure.
    ///
    /// Note tha twe only need this closure variant when:
    ///
    /// * the data is captured by ownership rather than as a reference, and
    /// * we want to return a reference.
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
    pub fn fun_ref<In, Out: ?Sized>(self, fun: fn(&Data, In) -> &Out) -> ClosureRef<Data, In, Out> {
        ClosureRef::new(self.0, fun)
    }

    /// Defines a `ClosureOptRef<Data, In, Out>` capturing `Data` and defining `In -> Option<&Out>` transformation.
    ///
    /// Consumes the `Capture` and moves the captured data inside the created closure.
    ///
    /// Note tha twe only need this closure variant when:
    ///
    /// * the data is captured by ownership rather than as a reference, and
    /// * we want to return an `Option` of a reference.
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
    pub fn fun_option_ref<In, Out: ?Sized>(
        self,
        fun: fn(&Data, In) -> Option<&Out>,
    ) -> ClosureOptRef<Data, In, Out> {
        ClosureOptRef::new(self.0, fun)
    }

    /// Defines a `ClosureResRef<Data, In, Out, Error>` capturing `Data` and defining `In -> Result<&Out, Error>` transformation.
    ///
    /// Consumes the `Capture` and moves the captured data inside the created closure.
    ///
    /// Note tha twe only need this closure variant when:
    ///
    /// * the data is captured by ownership rather than as a reference, and
    /// * we want to return a `Result` of a reference.
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
    pub fn fun_result_ref<In, Out: ?Sized, Error>(
        self,
        fun: fn(&Data, In) -> Result<&Out, Error>,
    ) -> ClosureResRef<Data, In, Out, Error> {
        ClosureResRef::new(self.0, fun)
    }

    /// Consumes the `Capture` and returns back the captured data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::*;
    ///
    /// let data = vec![42];
    /// let capture = Capture(data);
    ///
    /// let data_back = capture.into_captured_data();
    /// assert_eq!(vec![42], data_back);
    /// ```
    pub fn into_captured_data(self) -> Data {
        self.0
    }
}
