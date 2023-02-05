use crate::algaeset::AlgaeSet;
use crate::mapping::{BinaryOperation, PropertyError, PropertyType};

pub trait Magmoid<T: Copy + PartialEq> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T>;

    fn with(&mut self, left: T, right: T) -> Result<T, PropertyError> {
        self.binop().with(left, right)
    }
}

/// A set with an associated binary operation.
///
/// This is a representation of the simplest algebraic structure: the magma.
/// There are no specific properties required of its components, so its
/// construction involves nothing more than a set (specifically an
/// [`AlgaeSet`] and a binary operation (anything implementing
/// [`BinaryOperation`]).
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, AbelianOperation};
/// use algae_rs::magma::{Magmoid, Magma};
///
/// let mut add = AbelianOperation::new(&|a, b| a + b);
/// let mut magma = Magma::new(
///     AlgaeSet::<i32>::all(),
///     &mut add
/// );
///
/// let magma_sum = magma.with(1, 2);
/// assert!(magma_sum.is_ok());
/// assert!(magma_sum.unwrap() == 3);
/// ```
pub struct Magma<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
}

impl<'a, T> Magma<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>) -> Self {
        Self { aset, binop }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for Magma<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

/// A set equipped with a binary operation and a specified identity element.
///
/// [`UnitalMagma`] is a representation of the abstract algebraic unital magma.
/// The existence of an identity element is all that is required of its
/// binary operation. Its construction requires a set (specifically an
/// [`AlgaeSet`]) and an identity-preserving [`BinaryOperation`].
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, IdentityOperation};
/// use algae_rs::magma::{Magmoid, UnitalMagma};
///
/// let mut add = IdentityOperation::new(&|a, b| a + b, 0);
/// let mut magma = UnitalMagma::new(
///     AlgaeSet::<i32>::all(),
///     &mut add,
///     0
/// );
///
/// let sum = magma.with(1, 2);
/// assert!(sum.is_ok());
/// assert!(sum.unwrap() == 3);
///
/// let mut bad_add = IdentityOperation::new(&|a, b| a + b, 3);
/// let mut bad_magma = UnitalMagma::new(
///     AlgaeSet::<i32>::all(),
///     &mut bad_add,
///     3
/// );
///
/// let bad_sum = bad_magma.with(2, 3);
/// assert!(bad_sum.is_err());
/// ```
pub struct UnitalMagma<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
    identity: T,
}

impl<'a, T: Copy + PartialEq> UnitalMagma<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>, identity: T) -> Self {
        assert!(binop.is(PropertyType::WithIdentity(identity)));
        Self {
            aset,
            binop,
            identity,
        }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for UnitalMagma<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

impl<'a, T> Into<Magma<'a, T>> for UnitalMagma<'a, T> {
    fn into(self) -> Magma<'a, T> {
        Magma::new(self.aset, self.binop)
    }
}

/// A set equipped with an associative binary operation.
///
/// [`Groupoid`] is a representation of the abstract algebraic groupoid.
/// Associativity is all that is required of its binary operation: its
/// construction involves a set (specifically an [`AlgaeSet`]) and an
/// associative [`BinaryOperation`].
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, AssociativeOperation};
/// use algae_rs::magma::{Magmoid, Groupoid};
///
/// let mut add = AssociativeOperation::new(&|a, b| a + b);
/// let mut groupoid = Groupoid::new(
///     AlgaeSet::<i32>::all(),
///     &mut add
/// );
///
/// let groupoid_sum = groupoid.with(1, 2);
/// assert!(groupoid_sum.is_ok());
/// assert!(groupoid_sum.unwrap() == 3);
///
/// let mut div = AssociativeOperation::new(&|a, b| a / b);
/// let mut bad_groupoid = Groupoid::new(
///     AlgaeSet::<f32>::all(),
///     &mut div,
/// );
///
/// let ok_dividend = bad_groupoid.with(1.0, 2.0);
/// assert!(ok_dividend.is_ok());
/// assert!(ok_dividend.unwrap() == 0.5);
/// let err_dividend = bad_groupoid.with(3.0, 6.0);
/// assert!(err_dividend.is_err());
/// ```
pub struct Groupoid<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
}

impl<'a, T: Copy + PartialEq> Groupoid<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>) -> Self {
        assert!(binop.is(PropertyType::Associative));
        Self { aset, binop }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for Groupoid<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

impl<'a, T> Into<Magma<'a, T>> for Groupoid<'a, T> {
    fn into(self) -> Magma<'a, T> {
        Magma::new(self.aset, self.binop)
    }
}

/// A set equipped with a cancellative binary operation.
///
/// [`Quasigroup`] is a representation of the abstract algebraic quasigroup.
/// Cancellativity (ie. the Latin Square property) is required of its binary
/// operation. Its construction involves a set (specifically an [`AlgaeSet`])
/// and a [`BinaryOperation`] with the aforementioned properties.
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, CancellativeOperation};
/// use algae_rs::magma::{Magmoid, Quasigroup};
///
/// let mut add = CancellativeOperation::new(&|a, b| a + b);
/// let mut quasigroup = Quasigroup::new(
///     AlgaeSet::<i32>::all(),
///     &mut add
/// );
/// let sum = quasigroup.with(1, 2);
/// assert!(sum.is_ok());
/// assert!(sum.unwrap() == 3);
/// ```
pub struct Quasigroup<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
}

impl<'a, T: Copy + PartialEq> Quasigroup<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>) -> Self {
        assert!(binop.is(PropertyType::Cancellative));
        Self { aset, binop }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for Quasigroup<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

/// A set equipped with an associative binary operation with identity.
///
/// [`Monoid`] is a representation of the abstract algebraic monoid.
/// Associativity and identity are required of its binary operation. Its
/// construction involves a set (specifically an [`AlgaeSet`] and a
/// [`BinaryOperation`] with the aforementioned properties.
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, MonoidOperation};
/// use algae_rs::magma::{Magmoid, Monoid};
///
/// let mut add = MonoidOperation::new(&|a, b| a + b, 0);
/// let mut monoid = Monoid::new(
///     AlgaeSet::<i32>::all(),
///     &mut add,
///     0
/// );
///
/// let monoid_sum = monoid.with(1, 2);
/// assert!(monoid_sum.is_ok());
/// assert!(monoid_sum.unwrap() == 3);
///
/// let mut bad_add = MonoidOperation::new(&|a, b| a + b, 1);
/// let mut bad_monoid = Monoid::new(
///     AlgaeSet::<i32>::all(),
///     &mut bad_add,
///     1
/// );
///
/// let bad_monoid_sum = bad_monoid.with(1, 2);
/// assert!(bad_monoid_sum.is_err());
/// ```
pub struct Monoid<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
    identity: T,
}

impl<'a, T: Copy + PartialEq> Monoid<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>, identity: T) -> Self {
        assert!(binop.is(PropertyType::Associative));
        assert!(binop.is(PropertyType::WithIdentity(identity)));
        Self {
            aset,
            binop,
            identity,
        }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for Monoid<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}
