use std::fmt::Display;

use super::value_types::Quantity;

/// Resource has a value (amount) and a type
#[derive(PartialEq, Clone, Copy)]
pub struct Resource {
    pub(super) resource_type: ResourceType,
    pub(super) quantity: Quantity,
}

/// Resource types
#[derive(PartialEq, Clone, Copy)]
pub enum ResourceType {
    Wood,
    Gold,
}

impl Resource {
    /// Create a new Resource
    ///
    /// Params
    /// ---
    /// - resource_type: type of the resource
    ///
    /// Returns
    /// ---
    /// - New resource
    pub fn new(resource_type: ResourceType) -> Self {
        Resource {
            resource_type,
            quantity: 0,
        }
    }

    pub fn can_pay(&self, quantity: Quantity) -> bool {
        self.quantity - quantity >= 0
    }

    pub fn cannot_pay(&self) -> String {
        format!(
            "║{:^78}║\n",
            format!(
                "You don't have enough {} to perform this operation",
                &self.resource_type,
            ),
        )
    }

    /// Add a certain value to the resource
    ///
    ///
    /// Params
    /// ---
    /// - quantity: value which should be added
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the operation was successful
    /// - Err(String) with error description
    pub fn add(&mut self, quantity: Quantity) -> Result<(), String> {
        match quantity {
            0 => Err(format!(
                "║{:^78}║\n",
                format!("Cannot add 0 units of {}", &self,),
            )),
            n => {
                self.quantity += n;
                Ok(())
            }
        }
    }

    /// Subtract a certain value from the resource
    ///
    ///
    /// Params
    /// ---
    /// - quantity: value which should be subtracted
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the operation was successful
    /// - Err(String) with error description
    pub fn subtract(&mut self, quantity: Quantity) -> Result<(), String> {
        match self.can_pay(quantity) {
            true => {
                self.quantity -= quantity;
                Ok(())
            }
            false => Err(self.cannot_pay()),
        }
    }
}

/// for displaying resources
impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.resource_type)
    }
}

/// for displaying the resource type
impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Gold => {
                write!(f, "GOLD")
            }
            ResourceType::Wood => {
                write!(f, "WOOD")
            }
        }
    }
}
