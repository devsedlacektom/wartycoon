use super::{
    troops::{Unit, UnitType},
    value_types::{FighterPower, Quantity},
};
use std::collections::HashMap;

/// Game plan where the fields are stored
pub struct GamePlan {
    pub(super) fields: Vec<GameField>,
    pub(super) width: usize,
    pub(super) height: usize,
}

/// One game field which stores how many units have been sent to the field and its coordinates
pub struct GameField {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) units_occupying: Vec<UnitInField>,
}

/// Struct which stores how many units have been sent to the field
#[derive(Clone)]
pub struct UnitInField {
    pub owner: String,
    pub unit: Unit,
}

impl GamePlan {
    /// Create a new game plan instance with initialized fields
    ///
    /// Params
    /// ---
    /// - width: width of the battlefield
    /// - height: height of the battlefield
    pub fn new(width: usize, height: usize) -> Self {
        let mut fields_generated: Vec<GameField> = Vec::new();

        // generate plan with fields with coordinates
        let _ = (0..width).for_each(|x| {
            let _ = (0..height).for_each(|y| fields_generated.push(GameField::new(x, y)));
        });

        // return a new game plan with desired width and height
        Self {
            fields: fields_generated,
            width,
            height,
        }
    }

    /// Obtain mutable reference to a desired field on the battlefield,
    /// if the coordinates are within the battlefield dimensions
    ///
    /// Params
    /// ---
    /// - x: x coordinate on the battlefield
    /// - y: y coordinate on the battlefield
    ///
    /// Returns
    /// ---
    /// - Some(&mut field): mutable reference do desired field
    /// - None: if the field is not within range
    pub fn get_game_field(&mut self, x: usize, y: usize) -> Option<&mut GameField> {
        self.fields.get_mut(self.height * x + y)
    }

    /// Obtain dimensions of a field in a text format
    ///
    /// Returns
    /// ---
    /// - String with battlefield dimensions in text format
    pub fn get_dimensions(&self) -> String {
        let plural = if self.height == 1 { "" } else { "s" };
        format!("{} x {} field{}", self.width, self.height, plural)
    }

    /// Evaluate current state of the battlefield
    ///
    /// If the game has a winner, print their name and
    /// how many fields have they won
    pub fn evaluate(&self) {
        // get the fields which have a winner in them
        let evaluated_iterator = self
            .fields
            .iter()
            .map(|field| field.evaluate_field())
            .flatten();

        // used to store the number of wins
        let mut winner_frequency: HashMap<String, usize> = HashMap::new();

        // count number of winner references
        for winner in evaluated_iterator {
            *winner_frequency.entry(winner).or_insert(0) += 1;
        }

        // get player with highest number of won fields
        let highest_wins = winner_frequency
            .clone()
            .into_iter()
            .map(|(_, wins)| wins)
            .fold(0, |a, b| a.max(b));

        // find a possible winner
        let possible_winner = winner_frequency
            .clone()
            .into_iter()
            .find(|(_, wins)| *wins == highest_wins);

        match possible_winner {
            // winner was found
            Some((winner, wins)) => {
                // check if the winner is unique
                let is_unique = winner_frequency
                    .into_iter()
                    .filter(|(_, frequency)| *frequency == wins);

                // the length will be 1 if the winner is truly unique
                match is_unique.count() {
                    // winner unique
                    1 => println!(
                        "\nWinner of the game is {} with {} conquered fields\n",
                        winner, wins
                    ),
                    // more players with same number of conquered fields
                    n => println!(
                        "\nDraw! {} players have scored the same number of fields {}\n",
                        n, highest_wins
                    ),
                };
            }
            // no players with conquered fields
            None => println!("\nDraw! No player was able to win the most game fields!\n"),
        }
    }
}

impl GameField {
    /// Create a new instance of a game field with desired coordinates
    ///
    /// Params
    /// ---
    /// - x: x coordinate on the battlefield
    /// - y: y coordinate on the battlefield
    ///
    /// Returns
    /// ---
    /// - new instance of a game field with desired coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            units_occupying: Vec::new(),
        }
    }

    /// Adds units to the game field
    ///
    /// Params
    /// ---
    /// - units: which units to add
    pub fn add_units(&mut self, units: UnitInField) {
        self.units_occupying.push(units);
    }

    /// Return the quantity of a certain unit type that is currently
    /// occupying the field
    ///
    /// Params
    /// ---
    /// - unit_type: which unit_type to count
    ///
    /// Returns
    /// ---
    /// - quantity of a certain unit on the field
    fn get_units_by_type(&self, unit_type: UnitType) -> Quantity {
        self.units_occupying
            .iter()
            .filter(|field_unit| field_unit.unit.unit_type == unit_type)
            .map(|field_archer| field_archer.unit.quantity)
            .sum()
    }

    /// Evaluate who from the conquerors won the field
    ///
    /// Returns
    /// ---
    /// Some(name): if someone won the field
    /// None: if the field was conquered (either no one contested it, or could not decide)
    pub fn evaluate_field(&self) -> Option<String> {
        // map the power of players
        let units_frequency = self.units_occupying.iter().map(|unit_in_field| {
            (
                unit_in_field.owner.clone(),
                unit_in_field.unit.fighting_power(),
            )
        });

        // create a frequency storage
        let mut power_chart: HashMap<String, FighterPower> = HashMap::new();

        // sum the power of players
        for (owner, power) in units_frequency {
            *power_chart.entry(owner.clone()).or_insert(0.0) += power;
        }

        // find the highest power
        let highest_power = power_chart
            .clone()
            .into_iter()
            .map(|(_, power)| power)
            .fold(std::f64::MIN, |a, b| a.max(b));

        // find the winner (find which owner has the highest power, then return their name)
        let winner = power_chart
            .clone()
            .into_iter()
            .find(|(_, power)| (*power - highest_power).abs() < 0.1);

        // print winner of the field
        if let Some((winner_name, power)) = &winner {
            // check if the winner of the field is unique
            let is_unique = power_chart
                .into_iter()
                .filter(|(_, power)| (*power - highest_power).abs() < 0.1);

            // winner was not unique, field has no winner
            if is_unique.count() != 1 {
                return None;
            }

            // Winner IS UNIQUE:

            // this will give us a field with ONLY desired player's units
            let field = self.players_units(winner_name.clone());

            // get quantity of player's units
            let archer_units: Quantity = field.get_units_by_type(UnitType::Archer);
            let warrior_units: Quantity = field.get_units_by_type(UnitType::Warrior);
            let archer_plural = if archer_units == 1 { "" } else { "S" };
            let warrior_plural = if warrior_units == 1 { "" } else { "S" };

            // print who won the field
            println!("\nWinner of field ({}, {}) is {} with {} {}{}, {} {}{} and resulting fighting power of {:.2}\n",
                field.x, field.y, winner_name, archer_units, UnitType::Archer, archer_plural, warrior_units,
                UnitType::Warrior, warrior_plural, power
            );
        }

        winner.map(|(name, _)| name)
    }

    /// Return a copy of a game field, however only with units
    /// which are owned by a desired player
    ///
    /// Params
    /// ---
    /// - owner_nick: nick of the owner we want to filter units from
    pub fn players_units(&self, owner_nick: String) -> GameField {
        // filter which units to return
        let units: Vec<UnitInField> = self
            .units_occupying
            .iter()
            .filter(|unit| unit.owner == owner_nick)
            .cloned()
            .collect();

        // return filtered self
        GameField {
            x: self.x,
            y: self.y,
            units_occupying: units,
        }
    }
}

impl UnitInField {
    /// Create a new instance of a unit in field
    ///
    /// Params
    /// ---
    /// - unit_owner: owner of the unit, their nick
    /// - unit: unit that's sent on the field
    pub fn new(unit_owner: String, unit: Unit) -> Self {
        Self {
            owner: unit_owner,
            unit,
        }
    }
}
