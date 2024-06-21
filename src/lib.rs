#![doc = include_str!("../README.md")]
//!
//! The Rust User Standard Library
//!
//! This is an additions to the rust standard library. To
//! get started add it as a dependency:
//!
//! ```toml
//! [dependencies]
//! ext = "*"
//! ```
//!
//! At which point you can start coding and get acsess to all the great user
//! crates of the rust ecosystem. If you are looking for something you will
//! find that the structure is very similar to the rust standard so try looking
//! in the same module path and you might find something interesting.

#![forbid(
    unused_crate_dependencies,
    unsafe_code,
    missing_docs,
    missing_debug_implementations
)]

/// Like the [`Into`] and [`TryInto`] trait but is failable with no given reason. The reason to use
/// this would be when a conversion is not possible but the program should not stop beacuse of it.
pub trait SomeInto<T> {
    /// Converts the value into either something or nothing.
    fn some_into(self) -> Option<T>;
}

/// Like the [`From`] and [`TryFrom`] trait but is failable with no given reason. Is the mirror of
/// the [`SomeInto`] trait.
pub trait SomeFrom<T>: Sized {
    /// Converts the value into either something or nothing.
    fn some_from(value: T) -> Option<Self>;
}

impl<U, T: SomeFrom<U>> SomeInto<T> for U {
    #[must_use]
    fn some_into(self) -> Option<T> {
        T::some_from(self)
    }
}

/// a trait for chaining functions when you just need to make a call.
///
/// also useful when you need to destructure something.
pub trait MoveIt {
    /// calls a clojure on a type.
    ///
    /// ```rust
    /// # use core::ops::range;
    /// # use ext::parse::moveit;
    ///
    /// 3_usize.move_it(|v| (v, v+4))
    ///     .move_it(some)
    ///     .map(|(l, r)| l..r)
    ///     .unwrap()
    ///     .move_it(|range {start, end}| start + end);
    /// ```
    #[inline]
    fn move_it<F, U>(self, f: F) -> U
    where
        Self: Sized,
        F: FnOnce(Self) -> U,
    {
        f(self)
    }
}
impl<T> MoveIt for T {}

/// Gets the value out of an option returning early from the function with the defatult value if it
/// is [`None`]. This is best used in functions that return nothing but have effects on the object
/// they are called on.
#[macro_export]
macro_rules! t {
    ($e:expr) => {{
        let opt: Option<_> = $e;
        match opt {
            Some(t) => t,
            None => return Default::default(),
        }
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn can_early_return() {
        let a = Some(3);
        let b = t!(a);
        assert!(b == 3);
    }
}
