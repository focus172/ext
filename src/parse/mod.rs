pub extern crate bitflags;
pub extern crate regex;

pub trait SomeInto<T> {
    fn some_into(self) -> Option<T>;
}

pub trait SomeFrom<T>: Sized {
    fn some_from(value: T) -> Option<Self>;
}

impl<U, T: SomeFrom<U>> SomeInto<T> for U {
    #[must_use]
    fn some_into(self) -> Option<T> {
        T::some_from(self)
    }
}

/// A trait for chaining functions when you just need to make a call.
///
/// Also useful when you need to destructure something.
pub trait MoveIt {
    /// Calls a clojure on a type.
    ///
    /// ```rust
    /// # use core::ops::Range;
    /// # use ext::parse::MoveIt;
    ///
    /// 3_usize.move_it(|v| (v, v+4))
    ///     .move_it(Some)
    ///     .map(|(l, r)| l..r)
    ///     .unwrap()
    ///     .move_it(|Range {start, end}| start + end);
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
