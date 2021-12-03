use super::{
    actions::Actions,
    board::{GameField, GamePlan, UnitInField},
    buildings::Building,
    limits,
    properties::{HasCapacity, HasValue},
    resources::{
        Resource,
        ResourceType::{Gold, Wood},
    },
    troops::{Unit, UnitType},
    value_types::Quantity,
};

/// Player structure containing necessary information
#[derive(PartialEq, Clone)]
pub struct Player {
    pub nick: String,
    buildings: Vec<Building>,
    archers: Unit,
    warriors: Unit,
    wood: Resource,
    gold: Resource,
}

impl Player {
    /// Create new player structure
    ///
    /// Params
    /// ---
    /// - nick: Player's nickname
    ///
    /// Returns
    /// ---
    /// - New player instance
    pub fn new(nick: &str) -> Self {
        Player {
            nick: nick.into(),
            buildings: Vec::new(),
            archers: Unit::new(UnitType::Archer),
            warriors: Unit::new(UnitType::Warrior),
            wood: Resource::new(Wood),
            gold: Resource::new(Gold),
        }
    }

    /// Pays for an item (Reduces amount of a resource)
    ///
    /// Params
    /// ---
    /// - item: item that we want to pay for
    /// - quantity: how many of these items we want to pay for
    ///
    /// Returns
    /// ---
    /// - Ok(()) on successful payment
    /// - Err(String) containing details of what error occurred
    fn pay_for_item<T: HasValue>(&mut self, item: T, quantity: Quantity) -> Result<(), String> {
        // get item value
        let (wood, gold) = item.value();
        // get value we need to subtract
        let (wood, gold) = (wood * quantity, gold * quantity);

        // check if the player can pay for the item
        match self.wood.can_pay(wood) && self.gold.can_pay(gold) {
            true => {
                // "try" to subtract both -> will work because
                // we checked that it can be paid already
                self.wood.subtract(wood)?;
                self.gold.subtract(gold)?;

                Ok(())
            }
            // user cannot pay for the item
            false => {
                // Get wood error message, if user doesn't have enough wood
                let cannot_wood = match self.wood.can_pay(wood) {
                    true => "".into(),
                    false => self.wood.cannot_pay(),
                };

                // Get gold error message, if user doesn't have enough gold
                let cannot_gold = match self.gold.can_pay(gold) {
                    true => "".into(),
                    false => self.gold.cannot_pay(),
                };

                // if the gold was sufficient, only wood error is displayed, hence
                // the newline at the end of the message needs to be trimmed
                let cannot_wood = match cannot_gold.as_str() {
                    "" => cannot_wood.trim_end(),
                    _ => cannot_wood.as_str(),
                };

                // return formatted error
                Err(format!("{}{}", cannot_wood, cannot_gold.trim_end()))
            }
        }
    }

    /// Build a building of a desired type
    ///
    /// Params
    /// ---
    /// - building_type: type of a building to be built
    ///
    /// Returns
    /// ---
    /// - Ok(String) if a building was built successfully
    /// - Err(String) containing details of error that occurred while building the building
    fn build_a_building(&mut self, building_type: Building) -> Result<String, String> {
        // Check if the user can afford to build a building
        self.pay_for_item(building_type, 1)?;

        // create a new building of a desired type
        self.buildings.push(building_type);

        // success message
        Ok(format!(
            "║{:^78}║\n║{:^78}║",
            format!("Building of type {} was successfully built!", building_type),
            format!(
                "You currently have {} buildings of type {}.",
                self.number_of_buildings(building_type),
                building_type,
            )
        ))
    }

    /// Check if user has enough units to send
    ///
    /// Params
    /// ---
    /// - game_field: which field to send units to (used for error message)
    /// - unit_type: what type of unit to send
    /// - quantity: how many units of said type to send
    ///
    /// Returns
    /// ---
    /// - Ok(()) if units are available to send
    /// - Err(String) containing details of the problem
    fn enough_units_to_send(
        &self,
        game_field: &GameField,
        unit_type: UnitType,
        quantity: Quantity,
    ) -> Result<(), String> {
        // get current quantity
        let current_quantity = match unit_type {
            UnitType::Archer => self.archers.quantity,
            UnitType::Warrior => self.warriors.quantity,
        };

        // check if user has enough units
        if current_quantity < quantity {
            return Err(format!(
                "║{:^78}║\n║{:^78}║",
                format!(
                    "Cannot send {} units of type {} to occupy field ({},{}).",
                    quantity, unit_type, game_field.x, game_field.y,
                ),
                format!("Not enough units available ({}).", current_quantity,),
            ));
        }

        Ok(())
    }

    /// Perform action -> occupy desired field
    ///
    /// Params
    /// ---
    /// - game_field: desired field to occupy
    /// - unit_type: which unit type to choose
    /// - quantity: how many units of said type to send
    ///
    /// Returns
    /// - Ok(String) if troops were sent successfully
    /// - Err(String) if troops could not be sent
    ///               (field does not exist or user does not have enough units)
    fn occupy_fields(
        &mut self,
        game_field: Option<&mut GameField>,
        unit_type: UnitType,
        quantity: Quantity,
    ) -> Result<String, String> {
        // cannot access the game field
        if game_field.is_none() {
            return Err(format!(
                "║{:^78}║\n",
                "Sorry. Specified game field does not exist!",
            ));
        }

        // unwrapping after checking for none
        let game_field = game_field.unwrap();

        // check if user has enough units of said type to send (error can occur here)
        self.enough_units_to_send(game_field, unit_type, quantity)?;

        // create a copy of units that is sent to battlefield
        let unit_to_send = Unit::unit_to_send(unit_type, quantity);

        // send units to field
        game_field.add_units(UnitInField::new(self.nick.clone(), unit_to_send));

        // reduce number of available units
        match unit_type {
            UnitType::Archer => self.archers.send_occupy(quantity),
            UnitType::Warrior => self.warriors.send_occupy(quantity),
        }

        // Success string
        Ok(format!(
            "║{:^78}║\n║{:^78}║",
            format!(
                "{} units of type {} were successfully sent",
                quantity, unit_type,
            ),
            format!("to occupy field ({},{})!", game_field.x, game_field.y,),
        ))
    }

    /// Harvest crops from the surroundings of player's kingdom
    ///
    /// Returns
    /// - `Ok(String)` that the harvest was successful
    /// - Err(String) will never happen, the function is just compliant to the return type of other actions
    fn harvest(&mut self) -> Result<String, String> {
        // get the amount of gained crops
        let (wood, gold) = limits::HARVEST_GAIN;

        // add resources
        // this also will not fail, as we never get to add 0 resources to anything
        self.wood.add(wood)?;
        self.gold.add(gold)?;

        // return the formatted output
        Ok(format!(
            "║{:^78}║\n║{:^78}║\n║{:^78}║",
            "Harvest was a success!",
            format!("Gained {} wood and {} gold!", wood, gold,),
            format!(
                "Current warehouse supplies are: {}, {}.",
                self.wood, self.gold
            )
        ))
    }

    /// Get number of buildings of desired type
    ///
    /// Params
    /// ---
    /// - building_type: type of a desired building
    ///
    /// Returns
    /// ---
    /// - number of buildings of said type
    fn number_of_buildings(&self, building_type: Building) -> Quantity {
        self.buildings
            .iter()
            .filter(|building| **building == building_type)
            .map(|_| 1)
            .sum()
    }

    /// Get current fighters capacity
    ///
    /// Returns
    /// ---
    /// - current capacity to train fighters
    pub fn current_fighters_capacity(&self) -> Quantity {
        self.fighters_capacity() - self.archers.quantity - self.warriors.quantity
    }

    /// Return maximal capacity of warriors that can be stored in player's territory
    ///
    /// Returns
    /// ---
    /// - maximal fighters capacity
    fn fighters_capacity(&self) -> Quantity {
        self.buildings
            .iter()
            .filter(|building| **building == Building::Base)
            .map(|base| base.capacity())
            .sum()
    }

    /// Check if player has fighters available
    ///
    /// Returns
    /// ---
    /// - true: if there are some units available to send
    /// - false: otherwise
    pub fn has_fighters_available(&self) -> bool {
        self.archers.quantity + self.warriors.quantity > 0
    }

    /// Check fighters total capacity in bases
    ///
    /// Params
    /// ---
    /// - new_quantity: how many more fighters does user want to train
    ///
    /// Returns
    /// ---
    /// - Ok(()) on correct capacity
    /// - Err(String) containing details about the error that occurred
    fn check_fighters_capacity(&mut self, new_quantity: Quantity) -> Result<(), String> {
        // capacity exceeded
        if self.current_fighters_capacity() < new_quantity {
            return Err(format!(
                "║{:^78}║\n║{:^78}║\n║{:^78}║",
                "Cannot train new fighters, you picked too many units over capacity.",
                format!(
                    "{} picked, {} is total capacity.",
                    new_quantity,
                    self.fighters_capacity()
                ),
                "Consider building a new base instead!",
            ));
        }

        Ok(())
    }

    /// Trains units if there is enough capacity and enough resources available for training
    /// Only one type per round is permitted
    ///
    /// Params
    /// ---
    /// - unit_type: type of the trained unit
    /// - quantity: how many units are to be trained
    ///
    /// Returns
    /// ---
    /// - Ok(String) after successfully training the units
    /// - Err(String) containing error message
    fn train_units(&mut self, unit_type: UnitType, quantity: Quantity) -> Result<String, String> {
        // compute whether we are within capacity
        self.check_fighters_capacity(quantity)?;

        // try to pay for an item
        self.pay_for_item(unit_type, quantity)?;

        // train desired unit type
        match unit_type {
            UnitType::Archer => self.archers.train(quantity),
            UnitType::Warrior => self.warriors.train(quantity),
        }

        // language differences for plurals
        let quantity_string = if quantity == 1 { "unit" } else { "units" };
        let plural = if quantity == 1 { "" } else { "S" };

        // training was successful
        Ok(format!(
            "║{:^78}║",
            format!(
                "Training of {} {} of {}{} was successful",
                quantity, quantity_string, unit_type, plural
            ),
        ))
    }

    /// Performs a specified game action
    ///
    /// Params
    /// ---
    /// - action: which action to perform
    ///
    /// Returns
    /// ---
    /// - Ok(String) to print when everything went well,
    /// - Err(String) when an error occurred
    pub fn perform_action(
        &mut self,
        action: Actions,
        game_plan: &mut GamePlan,
    ) -> Result<String, String> {
        match action {
            Actions::Build(building) => self.build_a_building(building),
            Actions::Conquer(x, y, unit_type, quantity) => {
                self.occupy_fields(game_plan.get_game_field(x, y), unit_type, quantity)
            }
            Actions::Harvest => self.harvest(),
            Actions::Train(unit_type, quantity) => self.train_units(unit_type, quantity),
            _ => Ok("Unreachable statement".into()),
        }
    }

    /// Return formatted part of the table for player status
    /// which contains all user's units that have been sent on the battlefield
    ///
    /// Params
    /// ---
    /// - fields: vector of fields containing only user's troops
    ///
    /// Returns
    /// ---
    /// - formatted portion of user status' table
    fn occupied_fields(&self, fields: Vec<GameField>) -> String {
        // the header for the part of the table
        let header_string = format!("│ {:<29}│{:^47}│\n", "FIELDS OCCUPIED:", "");

        // format all fields containing user's troops
        let fields_string: Vec<String> = fields
            .iter()
            .map(|field| {
                let units_in_field: Vec<String> = field
                    .units_occupying
                    .iter()
                    .map(|unit_in_field| {
                        let plural = if unit_in_field.unit.quantity == 1 {
                            ""
                        } else {
                            "S"
                        };

                        format!(
                            "│{:^30}│{:>46} │",
                            "",
                            format!(
                                "{} {}{}",
                                unit_in_field.unit.quantity, unit_in_field.unit, plural
                            )
                        )
                    })
                    .collect();

                format!(
                    "│{:^30}│ {:<46}│\n{}\n",
                    "",
                    format!("FIELD ({},{}):", field.x, field.y),
                    units_in_field.join("\n"),
                )
            })
            .collect();

        // return formatted portion of the table
        format!(
            "{}{}",
            header_string,
            fields_string.join(&format!("│{:^30}├{}┤\n", "", "─".repeat(47)))
        )
    }

    /// Print player's status
    /// Generates a nice table used at the end of player's turn / when player asks for it
    ///
    /// Params
    /// ---
    /// - round: which round it currently is
    /// - game_plan: to gain data from the battlefield and print user's units
    /// - time_period: used in the header of the table to specify when
    ///                is the table relevant (f.e. at the end of the round)
    ///
    /// Returns
    /// ---
    /// - String containing formatted table (table is 80 characters wide)
    pub fn status(&self, round: usize, game_plan: &GamePlan, time_period: &str) -> String {
        // Format strings
        let line_top = format!("┌{}┐\n", "─".repeat(78));
        let line_middle_top = format!("├{}┬{}┤\n", "─".repeat(30), "─".repeat(47));
        let line_middle_center = format!("├{}┼{}┤\n", "─".repeat(30), "─".repeat(47));
        let line_bottom = format!("└{}┴{}┘\n", "─".repeat(30), "─".repeat(47));
        let empty_left_cell = " ".repeat(30);

        // auxiliary variables
        let plural_archers = if self.archers.quantity == 1 { "" } else { "S" };
        let plural_warriors = if self.warriors.quantity == 1 { "" } else { "S" };
        let plural_wood = if self.wood.quantity == 1 { "" } else { "S" };
        let plural_gold = if self.gold.quantity == 1 { "" } else { "S" };

        // get player's fields
        let players_fields: Vec<GameField> = game_plan
            .fields
            .iter()
            .clone()
            .map(|field| field.players_units(self.nick.to_string()))
            .filter(|field| !field.units_occupying.is_empty())
            .collect();

        // resulting string -> table of players current game status
        format!(
            "{}│{:^78}│\n{}{}{}{}{}{}{}{}{}{}{}{}",
            line_top,
            format!(
                "{}'s current statistics {} round {}",
                self.nick, time_period, round
            ),
            line_middle_top,
            format!(
                "│ {:<29}│{:^47}│\n",
                "BASE BUILDINGS:",
                format!("{}", self.number_of_buildings(Building::Base),)
            ),
            format!(
                "│{}│{:^47}│\n",
                empty_left_cell,
                format!(
                    "Currently used: {} / {} capacity",
                    self.archers.quantity + self.warriors.quantity,
                    self.fighters_capacity()
                ),
            ),
            line_middle_center,
            format!(
                "│ {:<29}│{:^47}│\n",
                "UNITS AVAILABLE:",
                format!(
                    "{} {}{}",
                    self.archers.quantity, self.archers, plural_archers,
                ),
            ),
            format!(
                "│{}│{:^47}│\n",
                empty_left_cell,
                format!(
                    "{} {}{}",
                    self.warriors.quantity, self.warriors, plural_warriors,
                ),
            ),
            line_middle_center,
            format!(
                "│ {:<29}│{:^47}│\n",
                "RESOURCES:",
                format!("{} WOODEN LOG{}", self.wood.quantity, plural_wood,),
            ),
            format!(
                "│{}│{:^47}│\n",
                empty_left_cell,
                format!("{} GOLDEN NUGGET{}", self.gold.quantity, plural_gold),
            ),
            line_middle_center,
            self.occupied_fields(players_fields),
            line_bottom
        )
    }

    /// Compute how many units of given type can user train at most
    ///
    /// Params
    /// ---
    /// - unit_type: type of the unit
    ///
    /// Returns
    /// ---
    /// - maximal number of units the user can train of given type
    pub fn train_max_units(&self, unit_type: UnitType) -> Quantity {
        let (unit_wood, unit_gold) = unit_type.value();

        // archers are only dependent on the gold
        match unit_type {
            UnitType::Archer => (self.gold.quantity / unit_gold).min(self.fighters_capacity()),
            UnitType::Warrior => (self.wood.quantity / unit_wood)
                .min(self.gold.quantity / unit_gold)
                .min(self.fighters_capacity()),
        }
    }

    /// Compute available units of given type to send out
    ///
    /// Params
    /// ---
    /// - unit_type: type of the unit
    ///
    /// Returns
    /// ---
    /// - currently available number of units of given type
    pub fn send_max_units(&self, unit_type: UnitType) -> Quantity {
        match unit_type {
            UnitType::Archer => self.archers.quantity,
            UnitType::Warrior => self.warriors.quantity,
        }
    }
}
