pub trait BinaryOperation {
    type Input; // type for the elements in the domain
    type Output; // type for the eleemnts in the codomain

    /// Gives the result of evaluating the operation with the given arguments.
    fn with(&self, a: Self::Input, b: Self::Input) -> Self::Output;
}

/// A function wrapper enforcing endomorphism conditions.
///
/// This is a light wrapper around a standard function (more specifically
/// anything implementing Fn(T, T) -> T) that enforces closure in the
/// mathematical sense. Operations wrapped by `ClosedOperation` must have two
/// inputs of the same type and have an output that exists in its domain.
///
/// ```
/// # use crate::algae::mappings::ClosedOperation;
/// # use crate::algae::mappings::BinaryOperation;
///
/// let add = ClosedOperation::new(
///     Box::new(|left: i32, right: i32| left + right)
/// );
/// assert_eq!(add.with(12, 13), 25);
/// ```
pub struct ClosedOperation<T> {
    pub op: Box<dyn Fn(T, T) -> T>,
}

impl<T> ClosedOperation<T> {
    /// Constructs a `ClosedOperation` from a heap-allocated function.
    pub fn new(op: Box<dyn Fn(T, T) -> T>) -> Self {
        Self { op }
    }
}

impl<T> Default for ClosedOperation<T> {
    /// Gives a `ClosedOperation` returning the left-most value passed to it.
    fn default() -> Self {
        Self {
            op: Box::new(|a, _| a),
        }
    }
}

impl<T> BinaryOperation for ClosedOperation<T> {
    type Input = T;
    type Output = T;

    fn with(&self, a: T, b: T) -> T {
        (self.op)(a, b)
    }
}
