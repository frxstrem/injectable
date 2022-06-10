use std::sync::Arc;

pub use injectable_macros::{inject, provide};

pub trait Provide<T> {
    fn provide(&self) -> T;
}

impl<T, P> Provide<T> for &P
where
    P: ?Sized + Provide<T>,
{
    fn provide(&self) -> T {
        P::provide(self)
    }
}

/// Alias for `Provide<Arc<T>>`.
pub trait ProvideArc<T>: Provide<Arc<T>> {}

impl<P, T> ProvideArc<T> for P where P: ?Sized + Provide<Arc<T>> {}
