/// One of the two variants.
///
/// # Examples
///
/// ```rust
/// use orx_closure::*;
///
/// let _ = OneOf2::<i32, bool>::Variant1(42);
/// let _ = OneOf2::<i32, bool>::Variant2(true);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneOf2<C1, C2> {
    /// First variant.
    Variant1(C1),
    /// Second variant.
    Variant2(C2),
}

/// One of the three variants.
///
/// # Examples
///
/// ```rust
/// use orx_closure::*;
///
/// let _ = OneOf3::<i32, bool, String>::Variant1(42);
/// let _ = OneOf3::<i32, bool, String>::Variant2(true);
/// let _ = OneOf3::<i32, bool, String>::Variant3("hi".to_string());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneOf3<C1, C2, C3> {
    /// First variant.
    Variant1(C1),
    /// Second variant.
    Variant2(C2),
    /// Third variant.
    Variant3(C3),
}

/// One of the four variants.
///
/// # Examples
///
/// ```rust
/// use orx_closure::*;
///
/// let _ = OneOf4::<i32, bool, String, char>::Variant1(42);
/// let _ = OneOf4::<i32, bool, String, char>::Variant2(true);
/// let _ = OneOf4::<i32, bool, String, char>::Variant3("hi".to_string());
/// let _ = OneOf4::<i32, bool, String, char>::Variant4('x');
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneOf4<C1, C2, C3, C4> {
    /// First variant.
    Variant1(C1),
    /// Second variant.
    Variant2(C2),
    /// Third variant.
    Variant3(C3),
    /// Fourth variant.
    Variant4(C4),
}
