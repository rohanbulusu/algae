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

#[derive(PartialEq)]
pub enum PropertyType {
    Commutative,
    Abelian,
    Associative
}

impl PropertyType {

    pub fn holds_over<'a, In: Copy, Out: PartialEq>(&self, op: &'a dyn Fn(In, In) -> Out, domain_sample: &Vec<In>) -> bool {
        match self {
            Self::Commutative | Self::Abelian => {
                permutations(domain_sample, 2).iter().all(|pair| {
                    let left = (op)(pair[0], pair[1]);
                    let right = (op)(pair[1], pair[0]);
                    left == right
                })
            },
            Self::Associative => {
                true
            }
        }
    }
    
}

pub trait AlgaeOperation<In: Copy, Out: PartialEq> {

    fn operation<'a>(&'a self) -> &'a dyn Fn(In, In) -> Out;

    fn properties(&self) -> Vec<PropertyType>;

    fn is(&self, property: PropertyType) -> bool {
        self.properties().contains(&property)
    }

    fn input_history(&self) -> &Vec<In>;

    fn cache(&mut self, input: In);

    fn with(&mut self, left: In, right: In) -> Result<Out, PropertyError> {
        self.cache(left);
        self.cache(right);
        for property in self.properties() {
            if property.holds_over(self.operation(), self.input_history()) {
                continue;
            }
            match property {
                PropertyType::Commutative | PropertyType::Abelian => {
                    return Err(PropertyError::CommutativityError);
                },
                PropertyType::Associative => {
                    return Err(PropertyError::AssociativityError);
                }
            }
        }
        return Ok((self.operation())(left, right));
    } 
    
}

/// A function wrapper enforcing commutativity.
///
/// Calling `with` 
///
/// # Examples
/// 
/// ```
/// # use algae::mapping::AbelianOperation;
/// # use algae::mapping::AlgaeOperation;
/// let mut add = AbelianOperation::new(&|a, b| {
///     a + b
/// });
/// 
/// let sum = add.with(1, 2);
/// assert!(sum.is_ok());
/// assert!(sum.unwrap() == 3);
/// 
/// let mut sub = AbelianOperation::new(&|a, b| {
///     a - b
/// });
/// 
/// let pos_difference = sub.with(4, 3);
/// assert!(pos_difference.is_err());
///
/// let neg_difference = sub.with(1, 2);
/// assert!(neg_difference.is_err());
/// ```
pub struct AbelianOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    history: Vec<T>
}

impl<'a, T> AbelianOperation<'a, T> {

    pub fn new(op: &'a dyn Fn(T, T) -> T) -> Self {
        Self {
            op,
            history: vec![]
        }
    }

}

impl<'a, T: Copy + PartialEq> AlgaeOperation<T, T> for AbelianOperation<'a, T> {

    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType> {
        vec![PropertyType::Commutative]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }

}

