use super::limits::{BASE_CAPACITY, BASE_COST};
use super::properties::{HasCapacity, HasValue};
use super::value_types::{Capacity, ResourceValue};
use std::fmt::Display;

/// Building types
#[derive(PartialEq, Clone, Copy)]
pub enum Building {
    Base,
}

/// Used for displaying the building
impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Building::Base => write!(f, "BASE"),
        }
    }
}

/// Every Building has a certain capacity
impl HasCapacity for Building {
    /// Return how many people can a building fit
    fn capacity(&self) -> Capacity {
        match &self {
            Self::Base => BASE_CAPACITY,
        }
    }
}

/// Every building can be purchased for a certain cost
impl HasValue for Building {
    /// Return how much a building costs
    fn value(&self) -> ResourceValue {
        match &self {
            Building::Base => BASE_COST,
        }
    }
}
