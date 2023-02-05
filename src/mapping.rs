fn permutations<T: Clone>(collection: &[T], group_size: usize) -> Vec<Vec<T>> {
    let mut groupings: Vec<Vec<T>> = vec![];
    for chunk in collection.chunks(group_size) {
        if chunk.len() != group_size {
            continue;
        }
        groupings.push(chunk.to_vec());
    }
    let mut reversed_collection = collection.to_vec();
    reversed_collection.reverse();
    for chunk in reversed_collection.chunks(group_size) {
        if chunk.len() != group_size {
            continue;
        }
        groupings.push(chunk.to_vec());
    }
    groupings
}

fn cayley_product<T: Copy>(collection: &Vec<T>) -> Vec<Vec<T>> {
    let mut pairs: Vec<Vec<T>> = vec![];
    for x in collection {
        for y in collection {
            pairs.push(vec![*x, *y]);
        }
    }
    pairs
}

#[derive(Debug)]
pub enum PropertyError {
    CommutativityError,
    AssociativityError,
    CancellativityError,
    IdentityError,
    Other(String),
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            PropertyError::CommutativityError => "Operation is not commutative!",
            PropertyError::AssociativityError => "Operation is not associative!",
            PropertyError::CancellativityError => "Operation is not cancellative!",
            PropertyError::IdentityError => "Operation has no valid identity!",
            PropertyError::Other(error) => error,
        };
        write!(f, "{}", msg)
    }
}

#[derive(PartialEq)]
pub enum PropertyType<T> {
    Commutative,
    Abelian,
    Associative,
    Cancellative,
    WithIdentity(T),
}

impl<T: Copy + PartialEq> PropertyType<T> {
    pub fn holds_over<'a>(&self, op: &'a dyn Fn(T, T) -> T, domain_sample: &Vec<T>) -> bool {
        match self {
            Self::Commutative | Self::Abelian => Self::commutativity_holds_over(op, domain_sample),
            Self::Associative => Self::associativity_holds_over(op, domain_sample),
            Self::Cancellative => Self::cancellative_holds_over(op, domain_sample),
            Self::WithIdentity(identity) => Self::identity_holds_over(op, domain_sample, *identity),
        }
    }

    fn commutativity_holds_over(op: &dyn Fn(T, T) -> T, domain_sample: &Vec<T>) -> bool {
        if domain_sample.len() < 2 {
            return true;
        }
        return permutations(domain_sample, 2).iter().all(|pair| {
            let left = (op)(pair[0], pair[1]);
            let right = (op)(pair[1], pair[0]);
            left == right
        });
    }

    fn associativity_holds_over(op: &dyn Fn(T, T) -> T, domain_sample: &Vec<T>) -> bool {
        if domain_sample.len() < 3 {
            return true;
        }
        return permutations(domain_sample, 3).iter().all(|triple| {
            let left_first = (op)((op)(triple[0], triple[1]), triple[2]);
            let right_first = (op)(triple[0], (op)(triple[1], triple[2]));
            left_first == right_first
        });
    }

    fn identity_holds_over(op: &dyn Fn(T, T) -> T, domain_sample: &[T], identity: T) -> bool {
        return domain_sample.iter().all(|e| {
            let from_left = (op)(identity, *e);
            let from_right = (op)(*e, identity);
            (*e == from_left) && (*e == from_right)
        });
    }

    fn cancellative_holds_over(op: &dyn Fn(T, T) -> T, domain_sample: &Vec<T>) -> bool {
        if domain_sample.len() < 3 {
            return true;
        }
        let left_cancellative = permutations(domain_sample, 3).iter().all(|triple| {
            if (op)(triple[0], triple[1]) == (op)(triple[0], triple[2]) {
                return triple[1] == triple[2];
            }
            true
        });
        let right_cancellative = permutations(domain_sample, 3).iter().all(|triple| {
            if (op)(triple[1], triple[0]) == (op)(triple[2], triple[0]) {
                return triple[1] == triple[2];
            }
            true
        });
        left_cancellative && right_cancellative
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
pub trait BinaryOperation<T: Copy + PartialEq> {
    /// Returns a reference to the function underlying the operation
    fn operation(&self) -> &dyn Fn(T, T) -> T;

    /// Vec of all enforced properties
    fn properties(&self) -> Vec<PropertyType<T>>;

    /// Returns whether or not `property` is enforced by the given operation
    fn is(&self, property: PropertyType<T>) -> bool {
        self.properties().contains(&property)
    }

    /// Returns a reference to a Vec of all previous inputs to the operation
    fn input_history(&self) -> &Vec<T>;

    /// Caches the given `input` to the operation's input history
    fn cache(&mut self, input: T);

    /// Returns the result of performing the given operation.
    ///
    /// If the operation is found not to obey all of its stated properties,
    /// an appropriate Err will be returned; if else, an Ok wrapping the
    /// proper result of the operation with the given inputs will be returned.
    fn with(&mut self, left: T, right: T) -> Result<T, PropertyError> {
        self.cache(left);
        self.cache(right);
        for property in self.properties() {
            if property.holds_over(self.operation(), self.input_history()) {
                continue;
            }
            match property {
                PropertyType::Commutative | PropertyType::Abelian => {
                    return Err(PropertyError::CommutativityError);
                }
                PropertyType::Associative => {
                    return Err(PropertyError::AssociativityError);
                }
                PropertyType::Cancellative => {
                    return Err(PropertyError::CancellativityError);
                }
                PropertyType::WithIdentity(_) => {
                    return Err(PropertyError::IdentityError);
                }
            }
        }
        return Ok((self.operation())(left, right));
    }
}

/// A function wrapper enforcing commutativity.
///
/// # Examples
///
/// ```
/// # use algae_rs::mapping::AbelianOperation;
/// # use algae_rs::mapping::BinaryOperation;
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
    history: Vec<T>,
}

impl<'a, T> AbelianOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T) -> Self {
        Self {
            op,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for AbelianOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<T>> {
        vec![PropertyType::Commutative, PropertyType::Abelian]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// A function wrapper enforcing associativity.
///
/// # Examples
///
/// ```
/// # use algae_rs::mapping::AssociativeOperation;
/// # use algae_rs::mapping::BinaryOperation;
/// let mut mul = AssociativeOperation::new(&|a, b| {
///     a * b
/// });
///
/// let six = mul.with(2, 3);
/// let twenty = mul.with(4, 5);
/// assert!(six.is_ok());
/// assert!(six.unwrap() == 6);
/// assert!(twenty.is_ok());
/// assert!(twenty.unwrap() == 20);
///
/// let mut div = AssociativeOperation::new(&|a, b| {
///     a / b
/// });
///
/// let whole_dividend = div.with(4.0, 2.0);
/// assert!(whole_dividend.is_ok());
/// assert!(whole_dividend.unwrap() == 2.0);
/// let fractional_dividend = div.with(3.0, 1.0);
/// assert!(fractional_dividend.is_err());
/// ```
pub struct AssociativeOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    history: Vec<T>,
}

impl<'a, T> AssociativeOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T) -> Self {
        Self {
            op,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for AssociativeOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<T>> {
        vec![PropertyType::Associative]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// A function wrapper enforcing identity existence.
///
/// # Examples
///
/// ```
/// # use algae_rs::mapping::IdentityOperation;
/// # use algae_rs::mapping::BinaryOperation;
/// let mut mul = IdentityOperation::new(&|a, b| {
///     a * b
/// }, 1);
///
/// let six = mul.with(2, 3);
/// assert!(six.is_ok());
/// assert!(six.unwrap() == 6);
///
/// let mut add = IdentityOperation::new(&|a, b| {
///     a + b
/// }, 3);
///
/// let sum = add.with(4, 2);
/// assert!(sum.is_err());
/// ```
pub struct IdentityOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    identity: T,
    history: Vec<T>,
}

impl<'a, T> IdentityOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T, identity: T) -> Self {
        Self {
            op,
            identity,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for IdentityOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<T>> {
        vec![PropertyType::WithIdentity(self.identity)]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// A function wrapper enforcing identity existence and associativity.
///
/// # Examples
///
/// ```
/// use algae_rs::mapping::IdentityOperation;
/// use algae_rs::mapping::{MonoidOperation, BinaryOperation};
///
/// let mut mul = MonoidOperation::new(&|a, b| a * b, 1);
///
/// let six = mul.with(2, 3);
/// assert!(six.is_ok());
/// assert!(six.unwrap() == 6);
///
/// let mut add = MonoidOperation::new(&|a, b| a + b, 3);
///
/// let sum = add.with(4, 2);
/// assert!(sum.is_err());
/// ```
pub struct MonoidOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    identity: T,
    history: Vec<T>,
}

impl<'a, T> MonoidOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T, identity: T) -> Self {
        Self {
            op,
            identity,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for MonoidOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<T>> {
        vec![
            PropertyType::Associative,
            PropertyType::WithIdentity(self.identity),
        ]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

#[cfg(test)]
mod tests {

    use super::{cayley_product, permutations};

    #[test]
    fn pair_permutations() {
        let v = &[1, 2, 3];
        let pairs = permutations(v, 2);
        assert!(pairs.contains(&vec![1, 2]));
        assert!(pairs.contains(&vec![3, 2]));
    }

    #[test]
    fn cayley_product_works() {
        let v = vec![1, 2, 3];
        let product = cayley_product(&v);
        assert!(
            product
                == vec![
                    vec![1, 1],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 1],
                    vec![2, 2],
                    vec![2, 3],
                    vec![3, 1],
                    vec![3, 2],
                    vec![3, 3]
                ]
        );
    }
}
