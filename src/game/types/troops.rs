use std::fmt::Display;

use super::{
    limits,
    properties::{HasPower, HasValue},
    value_types::{FighterPower, Quantity, ResourceValue},
};
/// Unit which can store a value
#[derive(Clone, Copy, PartialEq)]
pub struct Unit {
    pub(super) unit_type: UnitType,
    pub(super) quantity: Quantity,
}

/// Unit types
#[derive(Clone, Copy, PartialEq)]
pub enum UnitType {
    Warrior,
    Archer,
}

impl Unit {
    /// Create a new Unit
    ///
    /// Params
    /// ---
    /// - unit_type: type of the unit
    ///
    /// Returns
    /// ---
    /// - new instance of a unit with 0 people
    pub fn new(unit_type: UnitType) -> Self {
        Self {
            unit_type,
            quantity: 0,
        }
    }

    /// Create a new unit -> with specified quantity
    ///
    /// Used in combination with 'send_occupy' method
    ///
    /// Params
    /// ---
    /// - unit_type: type of the unit
    /// - quantity: number of units
    ///
    /// Returns
    /// ---
    /// - new unit instance (used for putting it in the field)
    pub fn unit_to_send(unit_type: UnitType, quantity: Quantity) -> Unit {
        Self {
            unit_type,
            quantity,
        }
    }

    /// Train new units
    ///
    /// Params
    /// ---
    /// - quantity: how many new units to train
    pub fn train(&mut self, quantity: Quantity) {
        self.quantity += quantity;
    }

    /// Decreases the quantity of a unit
    /// used in combination with 'unit_to_send' method
    /// to send troops to the battlefield
    pub fn send_occupy(&mut self, quantity: Quantity) {
        self.quantity -= quantity;
    }

    /// Return units fighting power
    pub fn fighting_power(&self) -> FighterPower {
        self.unit_type.power() * self.quantity as f64
    }
}

/// Every unit has its value
impl HasValue for Unit {
    /// Return value of a unit
    fn value(&self) -> ResourceValue {
        self.unit_type.value()
    }
}

/// Every Unit has a certain fighting power in the field
impl HasPower for UnitType {
    /// Return how much power a unit has
    fn power(&self) -> FighterPower {
        match &self {
            Self::Archer => limits::ARCHER_POWER,
            Self::Warrior => limits::WARRIOR_POWER,
        }
    }
}

/// Every Unit can be purchased for a certain cost
impl HasValue for UnitType {
    /// Return how much a unit type costs
    fn value(&self) -> ResourceValue {
        match &self {
            Self::Archer => limits::ARCHER_COST,
            Self::Warrior => limits::WARRIOR_COST,
        }
    }
}

/// for displaying unit types
impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Archer => {
                write!(f, "ARCHER")
            }
            UnitType::Warrior => {
                write!(f, "WARRIOR")
            }
        }
    }
}

/// for displaying unit type
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unit_type)
    }
}
