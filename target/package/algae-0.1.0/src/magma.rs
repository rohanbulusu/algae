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
/// use algae::algaeset::AlgaeSet;
/// use algae::mapping::{BinaryOperation, AbelianOperation};
/// use algae::magma::Magma; 
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
    binop: &'a mut dyn BinaryOperation<T>,
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
