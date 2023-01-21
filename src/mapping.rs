
pub trait Closed<T> {

	fn from_left(&self, a: T, b: T) -> T;

	fn from_right(&self, a: T, b: T) -> T;

}

pub trait Abelian<T: Clone + Copy + PartialEq>: Closed<T> {

	fn op(&self, a: T, b: T) -> T {
		assert!(self.from_left(a, b) == self.from_right(a, b));
		self.from_left(a, b)
	}

}

pub trait Associative<T>: Closed<T> {}

