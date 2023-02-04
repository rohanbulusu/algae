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

/// Common interface for all Algae operations.
///
/// All operations in Algae implement AlgaeOperation. This trait's key feature
/// is the provided `with` method, which provides a common interface both for 
/// retrieving the results of binary operations in Algae and for enforcing
/// their specified properties.
///
/// Property enforcement is done by keeping a history of all the inputs given
/// to the operation. The property is enforced among all combinations of 
/// previous inputs every time the operation is called. The existence of the
/// input history is required by `input_history`, and the caching mechanism is
/// given by `cache`. The operation itself is given by a reference to a
/// function via `operation`.
pub trait BinaryOperation<In: Copy, Out: PartialEq> {

    /// Returns a reference to the function underlying the operation
    fn operation<'a>(&'a self) -> &'a dyn Fn(In, In) -> Out;

    /// Vec of all enforced properties
    fn properties(&self) -> Vec<PropertyType>;

    /// Returns whether or not `property` is enforced by the given operation
    fn is(&self, property: PropertyType) -> bool {
        self.properties().contains(&property)
    }

    /// Returns a reference to a Vec of all previous inputs to the operation
    fn input_history(&self) -> &Vec<In>;

    /// Caches the given `input` to the operation's input history
    fn cache(&mut self, input: In);

    /// Returns the result of performing the given operation.
    /// 
    /// If the operation is found not to obey all of its stated properties,
    /// an appropriate Err will be returned; if else, an Ok wrapping the
    /// proper result of the operation with the given inputs will be returned.
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
/// # use algae::mapping::BinaryOperation;
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

impl<'a, T: Copy + PartialEq> BinaryOperation<T, T> for AbelianOperation<'a, T> {

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

