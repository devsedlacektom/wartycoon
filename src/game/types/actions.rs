use std::fmt::Display;

use super::{buildings::Building, troops::UnitType, value_types::Quantity};

/// Actions that can be performed in one game round
#[derive(PartialEq)]
pub enum Actions {
    Build(Building),
    Harvest,
    Train(UnitType, Quantity),
    Conquer(usize, usize, UnitType, Quantity), // x coordinate, y coordinate, unit type, quantity
    Quit,
}

/// Used for displaying actions in strings
impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actions::Build(building) => write!(f, "Build {}", building),
            Actions::Conquer(x, y, unit, quantity) => {
                let plural = if *quantity == 1 { "" } else { "S" };
                write!(
                    f,
                    "Conquer field ({},{}) with {} {}{}",
                    x, y, quantity, unit, plural
                )
            }
            Actions::Harvest => write!(f, "Harvest resources"),
            Actions::Quit => write!(f, "Quit game"),
            Actions::Train(unit, quantity) => {
                let plural = if *quantity == 1 { "" } else { "S" };
                write!(f, "Train {} {}{}", quantity, unit, plural)
            }
        }
    }
}
