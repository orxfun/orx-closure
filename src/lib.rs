#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod capture;
mod closure_opt_ref;
mod closure_ref;
mod closure_res_ref;
mod closure_val;
mod fun;
mod one_of;
mod one_of_variants;

pub use capture::Capture;
pub use closure_opt_ref::ClosureOptRef;
pub use closure_ref::ClosureRef;
pub use closure_res_ref::ClosureResRef;
pub use closure_val::Closure;
pub use one_of::{OneOf2, OneOf3, OneOf4};

pub use one_of_variants::one_of2::{
    closure_opt_ref::ClosureOptRefOneOf2, closure_ref::ClosureRefOneOf2,
    closure_res_ref::ClosureResRefOneOf2, closure_val::ClosureOneOf2,
};

pub use one_of_variants::one_of3::{
    closure_opt_ref::ClosureOptRefOneOf3, closure_ref::ClosureRefOneOf3,
    closure_res_ref::ClosureResRefOneOf3, closure_val::ClosureOneOf3,
};

pub use one_of_variants::one_of4::{
    closure_opt_ref::ClosureOptRefOneOf4, closure_ref::ClosureRefOneOf4,
    closure_res_ref::ClosureResRefOneOf4, closure_val::ClosureOneOf4,
};

pub use fun::{Fun, FunOptRef, FunRef, FunResRef};
