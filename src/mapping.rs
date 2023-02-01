
fn permutations<T: Clone>(collection: &Vec<T>, grouping_size: usize) -> Vec<Vec<T>> {
	let mut groupings: Vec<Vec<T>> = vec![];
	for chunk in collection.chunks(grouping_size) {
		groupings.push(chunk.to_vec());
	}
	groupings
}

#[derive(Debug)]
pub enum PropertyError {
	CommutativityError,
	AssociativityError,
	Other
}

impl std::fmt::Display for PropertyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", self)
	}
}


pub trait BinaryOperation<In: Copy, Out> {

	const ERROR: PropertyError;

	fn call_with(&self, a: In, b: In) -> Out;

	fn preserves_property(&self) -> bool;

	fn cache(&mut self, item: In);

	fn with(&mut self, a: In, b: In) -> Result<Out, PropertyError> {
		self.cache(a);
		self.cache(b);
		if !self.preserves_property() {
			return Err(Self::ERROR);
		}
		return Ok(self.call_with(a, b));
	}

}


pub struct AbelianOperation<T> {
	op: Box<dyn Fn(T, T) -> T>,
	cache: Vec<T>
}

impl<T: Copy> AbelianOperation<T> {

	pub fn new(op: Box<dyn Fn(T, T) -> T>) -> Self {
		Self {
			op,
			cache: vec![]
		}
	}

}

impl<T: Copy + PartialEq> BinaryOperation<T, T> for AbelianOperation<T> {

	const ERROR: PropertyError = PropertyError::CommutativityError;

	fn call_with(&self, a: T, b: T) -> T {
		(self.op)(a, b)
	}

	fn preserves_property(&self) -> bool {
		if self.cache.len() < 2 {
			return true;
		}
		permutations(&self.cache, 2).iter().all(|pair| {
			let (a, b) = (pair[0], pair[1]);
			self.call_with(a, b) == self.call_with(b, a)
		})
	}

	fn cache(&mut self, item: T) {
		if self.cache.contains(&item) {
			return;
		}
		return self.cache.push(item);
	}

}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn addition_is_abelian() {
		let mut add = AbelianOperation::<i32>::new(Box::new(|a, b| {
			a + b
		}));
		assert!(add.with(1, 2).is_ok());
		assert_eq!(add.with(1, 2).unwrap(), 3);
		assert!(add.with(3, 4).is_ok());
		assert_eq!(add.with(3, 4).unwrap(), 7);
	}

	#[test]
	fn subtraction_is_not_abelian() {
		let mut sub = AbelianOperation::<i32>::new(Box::new(|a, b| {
			a - b
		}));
		assert!(sub.with(0, 0).is_ok());
		assert_eq!(sub.with(0, 0).unwrap(), 0);
		assert!(sub.with(1, 2).is_err());
		assert!(sub.with(3, 4).is_err());
	}

}

