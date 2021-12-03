use super::value_types::{Capacity, FighterPower, ResourceValue};

// Define shared properties of different structures / enums

/// If the structure has a value (a cost)
/// this trait guarantees it can return its value (cost)
pub trait HasValue {
    fn value(&self) -> ResourceValue;
}

/// If the structure has some fighting power,
/// this trait guarantees it can return its power value
pub trait HasPower {
    fn power(&self) -> FighterPower;
}

/// If the structure has a certain capacity
/// this trait guarantees it can return its capacity value
pub trait HasCapacity {
    fn capacity(&self) -> Capacity;
}
