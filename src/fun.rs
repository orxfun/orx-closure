/// Function trait representing `In -> Out` transformation.
///
/// It provides the common interface for closures, such as `Closure<Capture, In, Out>`, over all capture types.
///
/// Furthermore, this trait enables to forget about the capture, or equivalently drop the `Capture` generic parameter, by using `dyn Fun<In, Out>` trait object.
///
/// # Relation with `Fn`
///
/// `Fun<In, Out>` can be considered equivalent to `Fn(In) -> Out`.
/// The reason it co-exists is that it is not possible to implement `fn_traits` in stable version.
///
/// However, all that implements `Fn(In) -> Out` also auto-implements `Fun<In, Out>`.
pub trait Fun<In, Out> {
    /// Calls the function with the given `input` and returns the produced output.
    fn call(&self, input: In) -> Out;
}
impl<In, Out, F: Fn(In) -> Out> Fun<In, Out> for F {
    fn call(&self, input: In) -> Out {
        self(input)
    }
}

/// Function trait representing `In -> &Out` transformation.
///
/// It provides the common interface for closures, such as `ClosureRef<Capture, In, Out>`, over all capture types.
///
/// Furthermore, this trait enables to forget about the capture, or equivalently drop the `Capture` generic parameter, by using `dyn FunRef<In, Out>` trait object.
///
/// # Relation with `Fn`
///
/// `FunRef<In, Out>` can be considered equivalent to `Fn(In) -> &Out`.
///
/// However, it appears to be impossible to have an instance of the latter due to lifetime errors.
/// Therefore, `FunRef<In, Out>` is required.
pub trait FunRef<In, Out: ?Sized> {
    /// Calls the function with the given `input` and returns the produced output.
    fn call(&self, input: In) -> &Out;
}

/// Function trait representing `In -> Option<&Out>` transformation.
///
/// It provides the common interface for closures, such as `ClosureOptRef<Capture, In, Out>`, over all capture types.
///
/// Furthermore, this trait enables to forget about the capture, or equivalently drop the `Capture` generic parameter, by using `dyn FunOptRef<In, Out>` trait object.
///
/// # Relation with `Fn`
///
/// `FunOptRef<In, Out>` can be considered equivalent to `Fn(In) -> Option<&Out>`.
///
/// However, it appears to be impossible to have an instance of the latter due to lifetime errors.
/// Therefore, `FunOptRef<In, Out>` is required.
pub trait FunOptRef<In, Out: ?Sized> {
    /// Calls the function with the given `input` and returns the produced output.
    fn call(&self, input: In) -> Option<&Out>;
}

/// Function trait representing `In -> Result<&Out, Error>` transformation.
///
/// It provides the common interface for closures, such as `ClosureResRef<Capture, In, Out>`, over all capture types.
///
/// Furthermore, this trait enables to forget about the capture, or equivalently drop the `Capture` generic parameter, by using `dyn FunOptRef<In, Out>` trait object.
///
/// # Relation with `Fn`
///
/// `FunResRef<In, Out, Error>` can be considered equivalent to `Fn(In) -> Result<&Out, Error>`.
///
/// However, it appears to be impossible to have an instance of the latter due to lifetime errors.
/// Therefore, `FunResRef<In, Out, Error>` is required.
pub trait FunResRef<In, Out: ?Sized, Error> {
    /// Calls the function with the given `input` and returns the produced output.
    fn call(&self, input: In) -> Result<&Out, Error>;
}
