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
    InvertibilityError,
    Other(String),
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            PropertyError::CommutativityError => "Operation is not commutative!",
            PropertyError::AssociativityError => "Operation is not associative!",
            PropertyError::CancellativityError => "Operation is not cancellative!",
            PropertyError::IdentityError => "Operation has no valid identity!",
            PropertyError::InvertibilityError => "Operation is not invertible!",
            PropertyError::Other(error) => error,
        };
        write!(f, "{msg}")
    }
}

pub enum PropertyType<'a, T> {
    Commutative,
    Abelian,
    Associative,
    Cancellative,
    WithIdentity(T),
    Invertible(T, &'a dyn Fn(T, T) -> T),
}

impl<'a, T: Copy + PartialEq> PropertyType<'a, T> {
    pub fn holds_over(&self, op: &dyn Fn(T, T) -> T, domain_sample: &Vec<T>) -> bool {
        match self {
            Self::Commutative | Self::Abelian => Self::commutativity_holds_over(op, domain_sample),
            Self::Associative => Self::associativity_holds_over(op, domain_sample),
            Self::Cancellative => Self::cancellative_holds_over(op, domain_sample),
            Self::WithIdentity(identity) => Self::identity_holds_over(op, domain_sample, *identity),
            Self::Invertible(identity, inv) => {
                Self::invertibility_holds_over(op, inv, domain_sample, *identity)
            }
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

    fn invertibility_holds_over(
        op: &dyn Fn(T, T) -> T,
        inv: &dyn Fn(T, T) -> T,
        domain_sample: &Vec<T>,
        identity: T,
    ) -> bool {
        if domain_sample.len() < 2 {
            return true;
        }
        return permutations(domain_sample, 2).iter().all(|pair| {
            let inverse_works = (inv)(pair[0], pair[0]) == identity;
            let left_composition_works = (inv)((op)(pair[0], pair[1]), pair[1]) == pair[0];
            let right_composition_works = (inv)((op)(pair[1], pair[0]), pair[1]) == pair[0];
            inverse_works && left_composition_works && right_composition_works
        });
    }
}

impl<'a, T> PartialEq for PropertyType<'a, T> {
    fn eq(&self, other: &PropertyType<'a, T>) -> bool {
        match self {
            Self::Commutative | Self::Abelian => {
                matches!(other, Self::Commutative) | matches!(other, Self::Abelian)
            }
            Self::Associative => matches!(other, Self::Associative),
            Self::Cancellative => matches!(other, Self::Cancellative),
            Self::WithIdentity(_) => matches!(other, Self::WithIdentity(_)),
            Self::Invertible(_, _) => matches!(other, Self::Invertible(_, _)),
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
pub trait BinaryOperation<T: Copy + PartialEq> {
    /// Returns a reference to the function underlying the operation
    fn operation(&self) -> &dyn Fn(T, T) -> T;

    /// Vec of all enforced properties
    fn properties(&self) -> Vec<PropertyType<'_, T>>;

    /// Returns whether or not `property` is enforced by the given operation
    fn is(&self, property: PropertyType<'_, T>) -> bool {
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
                PropertyType::Invertible(_, _) => {
                    return Err(PropertyError::InvertibilityError);
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

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
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

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
        vec![PropertyType::Associative]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// A function wrapper enforcing cancellativity.
///
/// # Examples
///
/// ```
/// use algae_rs::mapping::{CancellativeOperation, BinaryOperation};
///
/// let mut mul = CancellativeOperation::new(&|a, b| a * b);
///
/// let six = mul.with(2, 3);
/// assert!(six.is_ok());
/// assert!(six.unwrap() == 6);
/// ```
pub struct CancellativeOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    history: Vec<T>,
}

impl<'a, T> CancellativeOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T) -> Self {
        Self {
            op,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for CancellativeOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
        vec![PropertyType::Cancellative]
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
/// use algae_rs::mapping::{IdentityOperation, BinaryOperation};
///
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

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
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

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
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

/// A function wrapper enforcing identity existence and cancellativity.
///
/// # Examples
///
/// ```
/// use algae_rs::mapping::{LoopOperation, BinaryOperation};
///
/// let mut mul = LoopOperation::new(&|a, b| a * b, 1);
///
/// let six = mul.with(2, 3);
/// assert!(six.is_ok());
/// assert!(six.unwrap() == 6);
///
/// let mut add = LoopOperation::new(&|a, b| a + b, 3);
///
/// let sum = add.with(4, 2);
/// assert!(sum.is_err());
/// ```
pub struct LoopOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    identity: T,
    history: Vec<T>,
}

impl<'a, T> LoopOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T, identity: T) -> Self {
        Self {
            op,
            identity,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for LoopOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
        vec![
            PropertyType::Cancellative,
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

/// A function wrapper enforcing identity existence and invertibility.
///
/// # Examples
///
/// ```
/// use algae_rs::mapping::{InvertibleOperation, BinaryOperation};
///
/// let mut add = InvertibleOperation::new(&|a, b| a + b, &|a, b| a - b, 0);
///
/// let seven = add.with(4, 3);
/// assert!(seven.is_ok());
/// assert!(seven.unwrap() == 7);
///
/// let mut bad_add = InvertibleOperation::new(&|a, b| a + b, &|a, b| a * b, 0);
///
/// let sum = bad_add.with(4, 2);
/// assert!(sum.is_err());
/// ```
pub struct InvertibleOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    inv: &'a dyn Fn(T, T) -> T,
    identity: T,
    history: Vec<T>,
}

impl<'a, T> InvertibleOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T, inv: &'a dyn Fn(T, T) -> T, identity: T) -> Self {
        Self {
            op,
            inv,
            identity,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for InvertibleOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
        vec![
            PropertyType::WithIdentity(self.identity),
            PropertyType::Invertible(self.identity, self.inv),
        ]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// A function wrapper enforcing identity existence, invertibility, and associativity.
///
/// # Examples
///
/// ```
/// use algae_rs::mapping::{GroupOperation, BinaryOperation};
///
/// let mut add = GroupOperation::new(&|a, b| a + b, &|a, b| a - b, 0);
///
/// let seven = add.with(4, 3);
/// assert!(seven.is_ok());
/// assert!(seven.unwrap() == 7);
///
/// let mut bad_add = GroupOperation::new(&|a, b| a + b, &|a, b| a * b, 0);
///
/// let sum = bad_add.with(4, 2);
/// assert!(sum.is_err());
/// ```
pub struct GroupOperation<'a, T> {
    op: &'a dyn Fn(T, T) -> T,
    inv: &'a dyn Fn(T, T) -> T,
    identity: T,
    history: Vec<T>,
}

impl<'a, T> GroupOperation<'a, T> {
    pub fn new(op: &'a dyn Fn(T, T) -> T, inv: &'a dyn Fn(T, T) -> T, identity: T) -> Self {
        Self {
            op,
            inv,
            identity,
            history: vec![],
        }
    }
}

impl<'a, T: Copy + PartialEq> BinaryOperation<T> for GroupOperation<'a, T> {
    fn operation(&self) -> &dyn Fn(T, T) -> T {
        self.op
    }

    fn properties(&self) -> Vec<PropertyType<'_, T>> {
        vec![
            PropertyType::Associative,
            PropertyType::WithIdentity(self.identity),
            PropertyType::Invertible(self.identity, self.inv),
        ]
    }

    fn input_history(&self) -> &Vec<T> {
        &self.history
    }

    fn cache(&mut self, input: T) {
        self.history.push(input);
    }
}

/// Returns whether or not the given [`BinaryOperation`] has the [`PropertyType::Invertible`] property.
///
/// # Examples
///
/// ```
/// # use algae_rs::mapping::{BinaryOperation};
/// use algae_rs::mapping::{InvertibleOperation, AssociativeOperation, binop_is_invertible};
///
/// let add = InvertibleOperation::new(&|a: i32, b: i32| a + b, &|a: i32, b: i32| a - b, 0);
/// assert!(binop_is_invertible(&add));
///
/// let bad_add = AssociativeOperation::new(&|a: i32, b: i32| a * b);
/// assert!(!binop_is_invertible(&bad_add));
/// ```
pub fn binop_is_invertible<T: Copy + PartialEq>(binop: &dyn BinaryOperation<T>) -> bool {
    for property in binop.properties() {
        if let PropertyType::Invertible(_, _) = property {
            return true;
        }
    }
    false
}

/// Returns whether or not the given invertible [`BinaryOperation`] has the given `identity`.
///
/// # Examples
///
/// ```
/// # use algae_rs::mapping::{BinaryOperation};
/// use algae_rs::mapping::{InvertibleOperation, AssociativeOperation, binop_has_invertible_identity};
///
/// let add = InvertibleOperation::new(&|a: i32, b: i32| a + b, &|a: i32, b: i32| a - b, 0);
/// assert!(binop_has_invertible_identity(&add, 0));
///
/// let bad_add = InvertibleOperation::new(&|a: i32, b: i32| a + b, &|a: i32, b: i32| a - b, 123);
/// assert!(!binop_has_invertible_identity(&bad_add, 0));
/// ```
pub fn binop_has_invertible_identity<T: Copy + PartialEq>(
    binop: &dyn BinaryOperation<T>,
    identity: T,
) -> bool {
    assert!(binop_is_invertible(binop));
    for property in binop.properties() {
        if let PropertyType::Invertible(binop_identity, _) = property {
            return binop_identity == identity;
        }
    }
    false
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
