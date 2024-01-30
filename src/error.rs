use std::fmt::Display;

use color_eyre::eyre;

#[rustfmt::skip]
pub use {
    // `err!()` macro supports formatting like in `format!()`.
    eyre::eyre as err,
    // `result.wrap_err("text")`
    eyre::WrapErr,
};

/// Result type alias
pub type Result<T = ()> = color_eyre::Result<T>;

/// A wrapper around `eyre::ContextCompat`.
///
/// Similar to `WrapErr`, but for `Option` instead of `Result`.
pub trait WrapNone<T> {
    #[track_caller]
    fn wrap_none<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;

    #[track_caller]
    fn wrap_none_with<F, M>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> M,
        M: Display + Send + Sync + 'static;
}

impl<T> WrapNone<T> for Option<T> {
    fn wrap_none<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static,
    {
        eyre::ContextCompat::context(self, msg)
    }

    fn wrap_none_with<F, M>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> M,
        M: Display + Send + Sync + 'static,
    {
        eyre::ContextCompat::with_context(self, f)
    }
}
