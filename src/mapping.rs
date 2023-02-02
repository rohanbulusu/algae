fn permutations<T: Clone>(collection: &Vec<T>, group_size: usize) -> Vec<Vec<T>> {
    let mut groupings: Vec<Vec<T>> = vec![];
    for chunk in collection.chunks(group_size) {
        groupings.push(chunk.to_vec());
    }
    groupings
}

#[derive(Debug)]
pub enum PropertyError {
    CommutativityError,
    AssociativityError,
    Other(String),
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            PropertyError::CommutativityError => "Operation is not commutative!",
            PropertyError::AssociativityError => "Operation is not associative!",
            PropertyError::Other(error) => error,
        };
        write!(f, "{}", msg)
    }
}

pub trait OperationProperty<In, Out> {
    
    fn holds_over<'a>(&'a self, op: &'a dyn Fn(In, In) -> Out, domain_sample: &Vec<In>) -> bool;
    
    fn error(&self) -> PropertyError;
    
}

pub struct Commutative;

impl Commutative {

    pub fn new() -> Self {
        Self {}
    }
    
}

impl<In: Copy, Out: PartialEq> OperationProperty<In, Out> for Commutative {

    fn holds_over(&self, op: &dyn Fn(In, In) -> Out, domain_sample: &Vec<In>) -> bool {
        if domain_sample.len() < 2 {
            return true;
        }
        permutations(&domain_sample, 2).iter().all(|pair| {
            (op)(pair[0], pair[1]) == (op)(pair[1], pair[0])
        })
    }

    fn error(&self) -> PropertyError {
        PropertyError::CommutativityError
    }
    
}

pub trait BinaryOperation<In: Copy, Out> {
    
    fn properties<'a>(&'a self) -> Vec<Box<dyn OperationProperty<In, Out>>>;

    fn operation<'a>(&'a self) -> &'a dyn Fn(In, In) -> Out;

    fn history<'a>(&'a mut self) -> &'a mut Vec<In>;

    fn with(&mut self, left: In, right: In) -> Result<Out, PropertyError> {
        self.history().push(left);
        self.history().push(right);
        for property in self.properties() {
            let history = self.history().to_vec();
            if !property.holds_over(&self.operation(), &history) {
                return Err(property.error());
            }
        }
        Ok((self.operation())(left, right))
    }
    
}

pub struct AbelianOperation<T> {
    op: Box<dyn Fn(T, T) -> T>,
    cache: Vec<T>
}

impl<T> AbelianOperation<T> {

    pub fn new(op: Box<dyn Fn(T, T) -> T>) -> Self {
        Self { 
            op, 
            cache: vec![] 
        }
    }
    
}

impl<T: Copy + PartialEq> BinaryOperation<T, T> for AbelianOperation<T> {
    
    fn properties<'a>(&'a self) -> Vec<Box<dyn OperationProperty<T, T>>> {
        vec![Box::new(Commutative::new())]
    }

    fn operation<'a>(&'a self) -> &'a dyn Fn(T, T) -> T {
        &self.op
    }

    fn history<'a>(&'a mut self) -> &'a mut Vec<T> {
        &mut self.cache
    }

}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn addition_is_abelian() {
		let mut add = AbelianOperation::new(Box::new(|a, b| {
			a + b
		}));
		assert!(add.with(1, 2).is_ok());
		assert!(add.with(1, 2).unwrap() == 3);
		assert!(add.with(3, 4).is_ok());
		assert!(add.with(3, 4).unwrap() == 7);
	}

	#[test]
	fn subtraction_is_not_abelian() {
		let mut sub = AbelianOperation::new(Box::new(|a, b| {
			a - b
		}));
		assert!(sub.with(1, 2).is_err());
		assert!(sub.with(3, 4).is_err());
	}

}
