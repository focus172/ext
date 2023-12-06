pub use std::default::*;

pub fn default<T: std::default::Default>() -> T {
    T::default()
}
