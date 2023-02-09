
use crate::algaeset::AlgaeSet;
use crate::mapping::{PropertyType, BinaryOperation, binop_has_invertible_identity, binop_is_invertible};
use crate::magma::{Magmoid, Magma, UnitalMagma, Quasigroup};

/// A monoid with inverses.
///
/// [`Group`] is a representation of the abstract algebraic group.
/// Associativity, invertibility, and identity preservation are all required
/// of its binary operation. Its construction involves a set (specifically an
/// [`AlgaeSet`]) and a [`BinaryOperation`] with the aforementioned properties.
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, GroupOperation};
/// use algae_rs::magma::Magmoid;
/// use algae_rs::group::Group;
///
/// let mut add = GroupOperation::new(&|a, b| a + b, &|a, b| a - b, 0);
/// let mut group = Group::new(AlgaeSet::<i32>::all(), &mut add, 0);
///
/// let sum = group.with(1, 2);
/// assert!(sum.is_ok());
/// assert!(sum.unwrap() == 3);
///
/// let difference = group.with(1, -1);
/// assert!(difference.is_ok());
/// assert!(difference.unwrap() == 0);
///
/// let mut bad_add = GroupOperation::new(&|a, b| a + b, &|a, b| a * b, 0);
/// let mut bad_group = Group::new(AlgaeSet::<i32>::all(), &mut bad_add, 0);
///
/// let bad_sum = bad_group.with(3, 2);
/// assert!(bad_sum.is_err());
///
/// let bad_difference = bad_group.with(1, -1);
/// assert!(bad_difference.is_err());
/// ```
pub struct Group<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
    identity: T,
}

impl<'a, T: Copy + PartialEq> Group<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>, identity: T) -> Self {
        assert!(binop.is(PropertyType::Associative));
        assert!(binop.is(PropertyType::WithIdentity(identity)));
        assert!(binop_is_invertible(binop));
        assert!(binop_has_invertible_identity(binop, identity));
        Self {
            aset,
            binop,
            identity,
        }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for Group<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

impl<'a, T> From<Group<'a, T>> for Magma<'a, T> {
    fn from(group: Group<'a, T>) -> Magma<'a, T> {
        Magma::new(group.aset, group.binop)
    }
}

impl<'a, T: Copy + PartialEq> From<Group<'a, T>> for UnitalMagma<'a, T> {
    fn from(group: Group<'a, T>) -> UnitalMagma<'a, T> {
        UnitalMagma::new(group.aset, group.binop, group.identity)
    }
}

impl<'a, T: Copy + PartialEq> From<Group<'a, T>> for Quasigroup<'a, T> {
    fn from(group: Group<'a, T>) -> Quasigroup<'a, T> {
        Quasigroup::new(group.aset, group.binop)
    }
}

/// A commutative group
pub struct AbelianGroup<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
    identity: T
}

impl<'a, T: Copy + PartialEq> AbelianGroup<'a, T> {
    pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>, identity: T) -> Self {
        assert!(binop.is(PropertyType::Associative));
        assert!(binop.is(PropertyType::Commutative));
        assert!(binop.is(PropertyType::WithIdentity(identity)));
        assert!(binop_is_invertible(binop));
        assert!(binop_has_invertible_identity(binop, identity));
        Self {
            aset,
            binop,
            identity,
        }
    }
}

impl<'a, T: Copy + PartialEq> Magmoid<T> for AbelianGroup<'a, T> {
    fn binop(&mut self) -> &mut dyn BinaryOperation<T> {
        self.binop
    }
}

impl<'a, T> From<AbelianGroup<'a, T>> for Magma<'a, T> {
    fn from(group: AbelianGroup<'a, T>) -> Magma<'a, T> {
        Magma::new(group.aset, group.binop)
    }
}

impl<'a, T: Copy + PartialEq> From<AbelianGroup<'a, T>> for UnitalMagma<'a, T> {
    fn from(group: AbelianGroup<'a, T>) -> UnitalMagma<'a, T> {
        UnitalMagma::new(group.aset, group.binop, group.identity)
    }
}

impl<'a, T: Copy + PartialEq> From<AbelianGroup<'a, T>> for Quasigroup<'a, T> {
    fn from(group: AbelianGroup<'a, T>) -> Quasigroup<'a, T> {
        Quasigroup::new(group.aset, group.binop)
    }
}