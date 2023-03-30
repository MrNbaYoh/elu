#![warn(missing_docs)]

//! This crate provides traits to describe operations on EVAL-LINK-UPDATE data structures (similar to operations defined by Tarjan in ["Applications of Path Compression on Balanced Trees.”](https://doi.org/10.1145/322154.322161)).  
//! It also provides implementations of basic EVAL-LINK-UPDATE structures such as forest with path compression on evaluation (see [`CompressedForest`]).
//!
//! ## EVAL-LINK-UPDATE Operations
//! Suppose we have an associative operation ⊕. The three operations made available on forests are:
//! - [`EVAL`](EvalLinkUpdate::try_eval)`(n)`: find the root of the tree that contains the node `n`, let say `r`, and compute the product of all values on the path from `r` to `n` (i.e `value(r)` ⊕ ... ⊕ `value(n)`)
//! - [`LINK`](EvalLinkUpdate::try_link)`(n, m)`: find the root of the tree that contains the node `m`, let say `r`, and link it to the node `n` (i.e `r` becomes a child of `n`)
//! - [`UPDATE`](EvalLinkUpdate::try_update)`(n, v)`: find the root of the tree that contains the node `n`, let say `r`, and replace its value by `v`

mod forest;
pub use forest::CompressedForest;

mod node;

/// Collection of basic types that define standard associative operations.
pub mod operation;
pub use operation::AssociativeOperation;

/// An EVAL-LINK-UPDATE structure.
pub trait EvalLinkUpdate {
    /// The type used to identify nodes.
    type Id;
    /// The value type associated to nodes.
    type Value;
    /// The associative operation used by [`try_eval`](EvalLinkUpdate::try_eval) and [`eval`](EvalLinkUpdate::eval).
    type Operation: AssociativeOperation<Self::Value>;

    /// Creates a new tree root in the forest with the given value.
    fn new_root(&mut self, value: Self::Value) -> Self::Id;

    /// Computes the value of the node identified by `id`.
    ///
    /// # Errors
    /// Will return `Err` if [`Operation::associate`](AssociativeOperation::associate) fails.
    fn try_eval(
        &mut self,
        id: Self::Id,
    ) -> Result<Self::Value, <Self::Operation as AssociativeOperation<Self::Value>>::Error>;
    /// Infallible version of [`try_eval`](EvalLinkUpdate::try_eval). Requires [`Operation::Error`](EvalLinkUpdate::Operation) to be [`Infallible`](std::convert::Infallible).
    #[inline]
    fn eval(&mut self, id: Self::Id) -> Self::Value
    where
        Self::Operation: AssociativeOperation<Self::Value, Error = std::convert::Infallible>,
    {
        self.try_eval(id).unwrap()
    }
    /// Links the root of the tree that contains the node identified by `id_b` to the node identified by `id_a`.
    ///
    /// # Errors
    /// Will return `Err` if [`Operation::associate`](AssociativeOperation::associate) fails.
    fn try_link(
        &mut self,
        id_a: Self::Id,
        id_b: Self::Id,
    ) -> Result<(), <Self::Operation as AssociativeOperation<Self::Value>>::Error>;
    /// Infallible version of [`try_link`](EvalLinkUpdate::try_link). Requires [`Operation::Error`](EvalLinkUpdate::Operation) to be [`Infallible`](std::convert::Infallible).
    #[inline]
    fn link(&mut self, id_a: Self::Id, id_b: Self::Id)
    where
        Self::Operation: AssociativeOperation<Self::Value, Error = std::convert::Infallible>,
    {
        self.try_link(id_a, id_b).unwrap();
    }
    /// Updates the value of the root of the tree that contains the node identified by `id`.
    ///
    /// # Errors
    /// Will return `Err` if [`Operation::associate`](AssociativeOperation::associate) fails.
    fn try_update(
        &mut self,
        id: Self::Id,
        value: Self::Value,
    ) -> Result<(), <Self::Operation as AssociativeOperation<Self::Value>>::Error>;
    /// Infallible version of [`try_update`](EvalLinkUpdate::try_update). Requires [`Operation::Error`](EvalLinkUpdate::Operation) to be [`Infallible`](std::convert::Infallible).
    #[inline]
    fn update(&mut self, id: Self::Id, value: Self::Value)
    where
        Self::Operation: AssociativeOperation<Self::Value, Error = std::convert::Infallible>,
    {
        self.try_update(id, value).unwrap();
    }
}
