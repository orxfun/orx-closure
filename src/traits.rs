/// Function trait for `In -> Out` transformation.
///
/// Abstracts the captured types away from closures `Closure`, `ClosureOneOf2`, ...
///
/// *Nothing but `Fn(In) -> Out`, required as manual `Fn` implementations are not allowed.*
pub trait Fun<In, Out> {
    /// Calls the
    fn call(&self, input: In) -> Out;
}

/// Function trait for `In -> &Out` transformation.
///
/// Abstracts the captured types away from closures `ClosureRef`, `ClosureRefOneOf2`, ...
pub trait FunRef<In, Out> {
    fn call(&self, input: In) -> &Out;
}

/// Function trait for `In -> Option<&Out>` transformation.
///
/// Abstracts the captured types away from closures `ClosureOptRef`, `ClosureOptRefOneOf2`, ...
pub trait FunOptRef<In, Out> {
    fn call(&self, input: In) -> Option<&Out>;
}

/// Function trait for `In -> Result<&Out, Error>` transformation.
///
/// Abstracts the captured types away from closures `ClosureResRef`, `ClosureResRefOneOf2`, ...
pub trait FunResRef<In, Out, Error> {
    fn call(&self, input: In) -> Result<&Out, Error>;
}
