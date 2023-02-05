/// A representation of a ZF set.
///
/// All elements must belong to a "supertype" `E`. Subsets of the supertype are
/// determined by a given set of conditions (similar to the conditions used in
/// the set construction paradigm of traditional ZF set theory).
///
/// Element existence (ie. whether or not a certain element is a member of a
/// given set) is given through the [`has`](fn@AlgaeSet::has) function. Set
/// unions are given by the [`or`](fn@AlgaeSet::or) function, and set
/// intersections are given by the [`and`](fn@AlgaeSet::and) function.
///
/// # Examples
///
/// ```
/// use algae_rs::algaeset::AlgaeSet;
///
/// let mut pos_floats = AlgaeSet::new(vec![
///     Box::new(|e: f32| e > 0_f32)
/// ]);
///
/// assert!(pos_floats.has(12_f32));
///
/// let neg_floats = AlgaeSet::new(vec![
///     Box::new(|e: f32| e < 0_f32)
/// ]);
///
/// pos_floats.or(neg_floats);
/// let all_floats = pos_floats;
/// assert!(all_floats.has(12_f32));
/// assert!(all_floats.has(-12_f32));
/// ```
pub struct AlgaeSet<E> {
    pos_conditions: Vec<Box<dyn Fn(E) -> bool>>,
    neg_conditions: Vec<Box<dyn Fn(E) -> bool>>,
}

impl<E> AlgaeSet<E> {
    /// Returns an AlgaeSet defined by a `Vec` of conditions
    pub fn new(pos_conditions: Vec<Box<dyn Fn(E) -> bool>>) -> Self {
        Self {
            pos_conditions,
            neg_conditions: vec![],
        }
    }

    /// Returns an AlgaeSet defined by a single condition
    pub fn mono(condition: Box<dyn Fn(E) -> bool>) -> Self {
        Self::new(vec![condition])
    }

    /// Returns an AlgaeSet containing all members of the type `E`
    pub fn all() -> Self {
        Self {
            pos_conditions: vec![Box::new(|_x: E| true)],
            neg_conditions: vec![],
        }
    }
}

impl<E: Copy + Clone> AlgaeSet<E> {
    /// Returns whether or not `element` is in the given set
    pub fn has(&self, element: E) -> bool {
        if self.neg_conditions.iter().any(|c| (c)(element)) {
            return false;
        }
        return self.pos_conditions.iter().any(|c| (c)(element));
    }
}

impl<E: PartialEq + Copy + Clone + 'static> AlgaeSet<E> {
    /// Adds `element` to the given set
    pub fn add(&mut self, element: E) {
        self.neg_conditions.retain(|c| !(c)(element));
        self.pos_conditions.push(Box::new(move |x: E| x == element))
    }

    /// Removes `element` from the given set
    pub fn remove(&mut self, element: E) {
        self.pos_conditions.retain(|c| (c)(element));
        self.neg_conditions.push(Box::new(move |x: E| x == element))
    }

    /// Adds all elements from `other` to `self`
    pub fn or(&mut self, other: Self) {
        self.pos_conditions.push(Box::new(move |x: E| other.has(x)));
    }

    /// Removes all elements from `self` that aren't in `other`
    pub fn and(&mut self, other: Self) {
        self.neg_conditions
            .push(Box::new(move |x: E| !other.has(x)));
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    mod infinite_set {

        use super::*;

        #[derive(PartialEq, Clone, Copy)]
        enum Real {
            UInt(u32),
            SInt(i32),
            Float(f32),
        }

        #[test]
        fn has_element() {
            let REALS = AlgaeSet::<Real>::all();
            assert!(REALS.has(Real::UInt(12)));
            assert!(REALS.has(Real::SInt(-42)));
            assert!(REALS.has(Real::Float(-34.2)));
        }

        #[test]
        fn remove_element() {
            let mut REALS = AlgaeSet::<Real>::all();
            REALS.remove(Real::Float(23.1));
            assert!(REALS.has(Real::Float(23.2)));
            assert!(!REALS.has(Real::Float(23.1)));
        }

        #[test]
        fn add_after_remove() {
            let mut REALS = AlgaeSet::<Real>::all();
            REALS.remove(Real::Float(32.1));
            assert!(!REALS.has(Real::Float(32.1)));
            REALS.add(Real::Float(32.1));
            assert!(REALS.has(Real::Float(32.1)));
        }

        #[test]
        fn remove_after_add_after_remove() {
            let mut REALS = AlgaeSet::<Real>::all();
            assert!(REALS.has(Real::Float(32.1)));
            REALS.remove(Real::Float(32.1));
            assert!(!REALS.has(Real::Float(32.1)));
            REALS.add(Real::Float(32.1));
            assert!(REALS.has(Real::Float(32.1)));
            REALS.remove(Real::Float(32.1));
            assert!(!REALS.has(Real::Float(32.1)));
        }

        #[test]
        fn overlapping_union() {
            let REALS = AlgaeSet::<Real>::all();
            let mut FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            assert!(!FLOATS.has(Real::UInt(12)));
            FLOATS.or(REALS);
            assert!(FLOATS.has(Real::UInt(12)));
        }

        #[test]
        fn encompassing_union() {
            let mut REALS = AlgaeSet::<Real>::all();
            let FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            REALS.or(FLOATS);
            assert!(REALS.has(Real::Float(12.0)));
            assert!(REALS.has(Real::UInt(12)));
            assert!(REALS.has(Real::SInt(-12)));
        }

        #[test]
        fn disjoint_union() {
            let UINTS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => true,
                Real::SInt(_) => false,
                Real::Float(_) => false,
            }));
            let mut FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            assert!(FLOATS.has(Real::Float(12.0)));
            assert!(!FLOATS.has(Real::UInt(12)));
            FLOATS.or(UINTS);
            assert!(FLOATS.has(Real::Float(12.0)));
            assert!(FLOATS.has(Real::UInt(12)));
        }

        #[test]
        fn overlapping_intersection() {
            let REALS = AlgaeSet::<Real>::all();
            let mut FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            assert!(!FLOATS.has(Real::UInt(12)));
            FLOATS.and(REALS);
            assert!(!FLOATS.has(Real::UInt(12)));
        }

        #[test]
        fn encompassing_intersection() {
            let mut REALS = AlgaeSet::<Real>::all();
            let FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            assert!(REALS.has(Real::UInt(12)));
            assert!(REALS.has(Real::SInt(-12)));
            assert!(REALS.has(Real::Float(12.0)));
            REALS.and(FLOATS);
            assert!(REALS.has(Real::Float(12.0)));
            assert!(!REALS.has(Real::UInt(12)));
            assert!(!REALS.has(Real::SInt(-12)));
        }

        #[test]
        fn disjoint_intersection() {
            let UINTS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => true,
                Real::SInt(_) => false,
                Real::Float(_) => false,
            }));
            let mut FLOATS = AlgaeSet::<Real>::mono(Box::new(|x: Real| match x {
                Real::UInt(_) => false,
                Real::SInt(_) => false,
                Real::Float(_) => true,
            }));
            assert!(FLOATS.has(Real::Float(12.0)));
            assert!(!FLOATS.has(Real::UInt(12)));
            FLOATS.and(UINTS);
            assert!(!FLOATS.has(Real::Float(12.0)));
            assert!(!FLOATS.has(Real::UInt(12)));
        }
    }

    mod finite_set {

        use super::*;

        #[test]
        fn has_element() {
            let Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            assert!(Z2.has(1));
            assert!(Z2.has(0));
            assert!(!Z2.has(2));
            assert!(!Z2.has(-2));
        }

        #[test]
        fn add_element() {
            let mut Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            assert!(!Z2.has(2));
            Z2.add(2);
            assert!(Z2.has(2));
        }

        #[test]
        fn remove_element() {
            let mut Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            assert!(Z2.has(1));
            Z2.remove(1);
            assert!(!Z2.has(1));
        }

        #[test]
        fn overlapping_union() {
            let mut Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            let Z3 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 3 == x));
            Z2.or(Z3);
            assert!(Z2.has(0));
            assert!(Z2.has(1));
            assert!(Z2.has(2));
        }

        #[test]
        fn encompassing_union() {
            let Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            let mut Z3 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 3 == x));
            Z3.or(Z2);
            assert!(Z3.has(0));
            assert!(Z3.has(1));
            assert!(Z3.has(2));
        }

        #[test]
        fn disjoint_union() {
            let mut one = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 1));
            let two = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 2));
            one.or(two);
            assert!(one.has(1));
            assert!(one.has(2));
        }

        #[test]
        fn overlapping_intersection() {
            let mut Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            let one = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 1));
            Z2.and(one);
            assert!(Z2.has(1));
            assert!(!Z2.has(0));
        }

        #[test]
        fn encompassing_intersection() {
            let Z2 = AlgaeSet::<i32>::mono(Box::new(|x: i32| x % 2 == x));
            let mut one = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 1));
            one.and(Z2);
            assert!(one.has(1));
            assert!(!one.has(0));
        }

        #[test]
        fn disjoint_intersection() {
            let mut one = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 1));
            let two = AlgaeSet::<i32>::mono(Box::new(|x: i32| x == 2));
            one.and(two);
            assert!(!one.has(1));
            assert!(!one.has(2));
        }
    }
}
