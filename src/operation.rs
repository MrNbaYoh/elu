/// A trait for types that have a default associative operation.
pub trait WithDefaultOperation: Sized {
    /// The type returned in the event of an association error.
    type Error;

    /// Computes the association.
    ///
    /// # Errors
    /// Should return `Err` if the operation fails.
    fn default_associate(&self, rhs: &Self) -> Result<Self, Self::Error>;
}

/// A fallible associative operation.
pub trait AssociativeOperation<V> {
    /// The type returned in the event of an association error.
    type Error;

    /// Computes the association.
    ///
    /// # Errors
    /// Should return `Err` if the operation fails.
    fn associate(lhs: &V, rhs: &V) -> Result<V, Self::Error>;
}

/// A default associative operation for types that implements the [`WithDefaultOperation`] trait.
pub struct DefaultOperation;
impl<V> AssociativeOperation<V> for DefaultOperation
where
    V: WithDefaultOperation,
{
    type Error = V::Error;

    #[inline]
    fn associate(lhs: &V, rhs: &V) -> Result<V, V::Error> {
        lhs.default_associate(rhs)
    }
}

macro_rules! impl_infallible_clone_binop {
    ($name:ident, $trait:ident, $func:ident, $doc:expr) => {
        #[doc=$doc]
        #[derive(Debug)]
        pub struct $name;

        impl<V> AssociativeOperation<V> for $name
        where
            V: Clone + std::ops::$trait<V, Output = V>,
        {
            type Error = std::convert::Infallible;

            #[inline]
            fn associate(lhs: &V, rhs: &V) -> Result<V, Self::Error> {
                Ok(V::$func(lhs.clone(), rhs.clone()))
            }
        }
    };
}

macro_rules! impl_fallible_clone_binop {
    ($name:ident, $trait:ident, $func:ident, $doc:expr) => {
        #[doc=$doc]
        #[derive(Debug)]
        pub struct $name;

        impl<V, E> AssociativeOperation<V> for $name
        where
            V: Clone + std::ops::$trait<V, Output = Result<V, E>>,
        {
            type Error = E;

            #[inline]
            fn associate(lhs: &V, rhs: &V) -> Result<V, Self::Error> {
                V::$func(lhs.clone(), rhs.clone())
            }
        }
    };
}

impl_infallible_clone_binop!(CloneAdd, Add, add, "Clone and add operation.");
impl_infallible_clone_binop!(CloneMul, Mul, mul, "Clone and multiply operation.");
impl_infallible_clone_binop!(CloneBitOr, BitOr, bitor, "Clone and bitwise or operation.");
impl_infallible_clone_binop!(
    CloneBitAnd,
    BitAnd,
    bitand,
    "Clone and bitwise and operation."
);
impl_infallible_clone_binop!(
    CloneBitXor,
    BitXor,
    bitxor,
    "Clone and bitwise xor operation."
);

impl_fallible_clone_binop!(
    FallibleCloneAdd,
    Add,
    add,
    "Fallible version of [`CloneAdd`]."
);
impl_fallible_clone_binop!(
    FallibleCloneMul,
    Mul,
    mul,
    "Fallible version of [`CloneMul`]."
);
impl_fallible_clone_binop!(
    FallibleCloneBitOr,
    BitOr,
    bitor,
    "Fallible version of [`CloneBitOr`]."
);
impl_fallible_clone_binop!(
    FallibleCloneBitAnd,
    BitAnd,
    bitand,
    "Fallible version of [`CloneBitAnd`]."
);
impl_fallible_clone_binop!(
    FallibleCloneBitXor,
    BitXor,
    bitxor,
    "Fallible version of [`CloneBitXor`]."
);
