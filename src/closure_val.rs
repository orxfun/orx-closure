use std::fmt::Debug;

use crate::fun::Fun;

/// Closure strictly separating the captured data from the function, and hence, having two components:
///
/// * `Capture` is any captured data,
/// * `fn(&Capture, In) -> Out` is the transformation.
///
/// It represents the transformation `In -> Out`.
///
/// Note that, unlike trait objects of fn-traits, `Closure` auto-implements `Clone` given that captured data is cloneable.
///
/// # Example
///
/// ```rust
/// use orx_closure::*;
///
/// let name = String::from("morgana");
///
/// // nth_char: Closure<String, usize, Option<char>>
/// let nth_char = Capture(name).fun(|n, i| n.chars().nth(i));
///
/// assert_eq!(Some('m'), nth_char.call(0));
///
/// // alternatively
/// let fun = nth_char.as_fn();
/// assert_eq!(Some('g'), fun(3));
/// ```
#[derive(Clone)]
pub struct Closure<Capture, In, Out> {
    capture: Capture,
    fun: fn(&Capture, In) -> Out,
}

impl<Capture: Debug, In, Out> Debug for Closure<Capture, In, Out> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Closure")
            .field("capture", &self.capture)
            .finish()
    }
}

impl<Capture, In, Out> Closure<Capture, In, Out> {
    pub(super) fn new(capture: Capture, fun: fn(&Capture, In) -> Out) -> Self {
        Self { capture, fun }
    }

    /// Calls the closure with the given `input`.
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
    #[inline(always)]
    pub fn call(&self, input: In) -> Out {
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
    /// let data = ExpensiveData(vec![0, 1, 2]);
    ///
    /// let get_number = Capture(data).fun(|data, i| 42 + data.0[i]);
    ///
    /// assert_eq!(42, get_number.call(0));
    /// assert_eq!(44, get_number.call(2));
    ///
    /// let _data: ExpensiveData = get_number.into_captured_data();
    /// ```
    pub fn into_captured_data(self) -> Capture {
        self.capture
    }

    /// Returns the closure as an `impl Fn(In) -> Out` struct, allowing the convenience
    ///
    /// * to avoid the `call` method,
    /// * or pass the closure to functions accepting a function generic over the `Fn`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_closure::Capture;
    ///
    /// let base = 2;
    /// let modulo = Capture(base).fun(|b, n| n % b);
    ///
    /// // with call method
    /// assert_eq!(0, modulo.call(42));
    /// assert_eq!(1, modulo.call(7));
    ///
    /// // getting it as 'Fn' and directly calling the closure
    /// let module_fn = modulo.as_fn();
    /// assert_eq!(0, module_fn(42));
    /// assert_eq!(1, module_fn(7));
    /// ```
    pub fn as_fn(&self) -> impl Fn(In) -> Out + '_ {
        |x| self.call(x)
    }
}

impl<Capture, In, Out> Fun<In, Out> for Closure<Capture, In, Out> {
    fn call(&self, input: In) -> Out {
        Closure::call(self, input)
    }
}
