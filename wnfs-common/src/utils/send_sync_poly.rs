/// A conditionally compiled atomic or non-atomic reference counter.
/// In this target it's backed by `std::sync::Arc<T>`.
#[cfg(not(target_arch = "wasm32"))]
pub type Arc<T> = std::sync::Arc<T>;

/// A conditionally compiled atomic or non-atomic reference counter.
/// In this target it's backed by `std::rc::Rc<T>`.
#[cfg(target_arch = "wasm32")]
pub type Arc<T> = std::rc::Rc<T>;

/// A conditionally compiled `BoxFuture`.
/// Resolves to either `BoxFuture` or `LocalBoxFuture`, depending on the target.
#[cfg(not(target_arch = "wasm32"))]
pub type BoxFuture<'a, T> = futures::future::BoxFuture<'a, T>;

/// A conditionally compiled `BoxFuture`.
/// Resolves to either `BoxFuture` or `LocalBoxFuture`, depending on the target.
#[cfg(target_arch = "wasm32")]
pub type BoxFuture<'a, T> = futures::future::LocalBoxFuture<'a, T>;

/// A conditionally compiled `BoxStream`.
/// Resolves to either `BoxStream` or `LocalBoxStream`, depending on the target.
#[cfg(not(target_arch = "wasm32"))]
pub type BoxStream<'a, T> = futures::stream::BoxStream<'a, T>;

/// A conditionally compiled `BoxStream`.
/// Resolves to either `BoxStream` or `LocalBoxStream`, depending on the target.
#[cfg(target_arch = "wasm32")]
pub type BoxStream<'a, T> = futures::stream::LocalBoxStream<'a, T>;

/// A conditionally compiled trait indirection for `Send` bounds.
/// This target makes it require `Send`.
#[cfg(not(target_arch = "wasm32"))]
pub trait CondSend: Send {}

/// A conditionally compiled trait indirection for `Send` bounds.
/// This target makes it not require any marker traits.
#[cfg(target_arch = "wasm32")]
pub trait CondSend {}

#[cfg(not(target_arch = "wasm32"))]
impl<S> CondSend for S where S: Send {}

#[cfg(target_arch = "wasm32")]
impl<S> CondSend for S {}

/// A conditionally compiled trait indirection for `Send + Sync` bounds.
/// This target makes it require `Send + Sync`.
#[cfg(not(target_arch = "wasm32"))]
pub trait CondSync: Send + Sync {}

/// A conditionally compiled trait indirection for `Send + Sync` bounds.
/// This target makes it not require any marker traits.
#[cfg(target_arch = "wasm32")]
pub trait CondSync {}

#[cfg(not(target_arch = "wasm32"))]
impl<S> CondSync for S where S: Send + Sync {}

#[cfg(target_arch = "wasm32")]
impl<S> CondSync for S {}
