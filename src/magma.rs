use crate::algaeset::AlgaeSet;
use crate::mapping::{BinaryOperation, PropertyType};

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
/// use algae_rs::magma::Magma; 
/// 
/// let mut add = AbelianOperation::new(&|a, b| a + b);
/// let mut magma = Magma::new(
/// 	AlgaeSet::<i32>::all(), 
/// 	&mut add
/// );
///
/// let magma_sum = magma.with(1, 2);
/// assert!(magma_sum.is_ok());
/// assert!(magma_sum.unwrap() == 3);
/// ```
pub struct Magma<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>
}

impl<'a, T> Magma<'a, T> {

	pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>) -> Self {
		Self {
			aset,
			binop
		}
	}

}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for Magma<'a, T> {

	fn operation(&self) -> &dyn Fn(T, T) -> T {
		self.binop.operation()
	}

	fn properties(&self) -> Vec<PropertyType<T>> {
		self.binop.properties()
	}

	fn input_history(&self) -> &Vec<T> {
		self.binop.input_history()
	}

	fn cache(&mut self, input: T) {
		self.binop.cache(input);
	}


}

/// A set equipped with an associative binary operation with identity.
///
/// This is a representation of the monoids of abstract algebra.
/// Associativity and identity are required of its binary operation; its
/// construction involves a set (specifically an [`AlgaeSet`] and a binary 
/// operation (anything implementing [`BinaryOperation`]) with the
/// aforementioned properties.
///
/// # Examples
/// 
/// ```
/// use algae_rs::algaeset::AlgaeSet;
/// use algae_rs::mapping::{BinaryOperation, MonoidOperation};
/// use algae_rs::magma::Monoid; 
/// 
/// let mut add = MonoidOperation::new(&|a, b| a + b, 0);
/// let mut monoid = Monoid::new(
///     AlgaeSet::<i32>::all(), 
///     &mut add,
/// 	0
/// );
///
/// let monoid_sum = monoid.with(1, 2);
/// assert!(monoid_sum.is_ok());
/// assert!(monoid_sum.unwrap() == 3);
/// 
/// let mut bad_add = MonoidOperation::new(&|a, b| a + b, 1);
/// let mut bad_monoid = Monoid::new(
/// 	AlgaeSet::<i32>::all(),
/// 	&mut bad_add,
/// 	1
/// );
/// 
/// let bad_monoid_sum = bad_monoid.with(1, 2);
/// assert!(bad_monoid_sum.is_err());
/// ```
pub struct Monoid<'a, T> {
    aset: AlgaeSet<T>,
    binop: &'a mut dyn BinaryOperation<T>,
    identity: T
}

impl<'a, T: Copy + PartialEq> Monoid<'a, T> {

	pub fn new(aset: AlgaeSet<T>, binop: &'a mut dyn BinaryOperation<T>, identity: T) -> Self {
		assert!(binop.is(PropertyType::Associative));
		assert!(binop.is(PropertyType::WithIdentity(identity)));
		Self {
			aset,
			binop,
			identity
		}
	}

}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for Monoid<'a, T> {

	fn operation(&self) -> &dyn Fn(T, T) -> T {
		self.binop.operation()
	}

	fn properties(&self) -> Vec<PropertyType<T>> {
		self.binop.properties()
	}

	fn input_history(&self) -> &Vec<T> {
		self.binop.input_history()
	}

	fn cache(&mut self, input: T) {
		self.binop.cache(input);
	}


}