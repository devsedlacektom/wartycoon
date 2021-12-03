use super::notifications::{print_help, print_rules};
use super::types::limits::{DEFAULT_PLAN_HEIGHT, DEFAULT_PLAN_WIDTH};
use super::types::{
    actions::Actions, board::GamePlan, buildings::Building, player::Player, troops::UnitType,
};
use super::user_input::get_line;

/// Confirm an action from user
/// Prints a confirmation message and asks user to confirm, that they want to do the action.
pub fn confirm_action(action: &Actions) -> bool {
    loop {
        // ask user to confirm action
        println!("\nPlease confirm this action: {}\n(Either press enter or type 'yes' or 'y', or decline by typing 'no' or 'n'.)", action);

        // get a line and trim it
        let line = get_line();
        let line = line.trim();

        // check what it said
        match line {
            "YES" | "Yes" | "yes" | "Y" | "y" | "" => return true,
            "NO" | "No" | "no" | "N" | "n" => return false,
            _ => continue,
        }
    }
}

/// Get the conquer action
///
/// Params
/// ---
/// - player: Reference to player (for aid, how many units can player train
/// - x: x coordinate
/// - y: y coordinate
///
/// Returns
/// ---
/// - Some(conquer_action): if user decided to conquer a field
/// - None: if the user chose to leave the conquer action specification
fn get_conquer_action(player: &Player, x: usize, y: usize) -> Option<Actions> {
    units_action(player, UnitAction::Conquer(x, y))
}

/// Get the training action
///
/// Params
/// ---
/// - player: Reference to player (for aid, how many units can player train)
///
/// Returns
/// ---
/// - Some(training_action): if user decided to train units
/// - None: if user chose to leave the training action specification
fn get_train_action(player: &Player) -> Option<Actions> {
    units_action(player, UnitAction::Train)
}

/// Get the player's action
/// Serves to get input from the user and turn it to an action (defined in types.rs)
///
/// Params
/// ---
/// - player: player reference
/// - game_plan: game plan reference (for printing of current status)
/// - round: which round is currently
///
/// Returns
/// ---
/// - Actions: what action has user decided to perform
pub fn get_player_action(player: &Player, game_plan: &GamePlan, round: usize) -> Actions {
    // input loop
    loop {
        println!(
            "\nRound {}, {}'s action (for help please, type '6' or 'help'):\n",
            round, player.nick
        );

        // get the line, trim it
        let line_one = get_line();
        let line_one = line_one.trim();

        // parse the contents of the line
        match line_one {
            "1" | "build" | "Build" | "BUILD" => return Actions::Build(Building::Base),
            "2" | "harvest" | "Harvest" | "HARVEST" => return Actions::Harvest,
            "3" | "train" | "Train" | "TRAIN" => match get_train_action(player) {
                Some(action) => return action,
                None => {
                    println!("\nNo worries, no units were trained!\n");
                }
            },
            "4" | "conquer" | "Conquer" | "CONQUER" => {
                // putting coordinates 0,0 as this is the default behavior,
                // in case the custom game mode is implemented, there will be additional
                // input handling to just simply call this function with the input.
                // until then, this might seem unnecessary
                match get_conquer_action(player, DEFAULT_PLAN_WIDTH - 1, DEFAULT_PLAN_HEIGHT - 1) {
                    Some(action) => return action,
                    None => {
                        println!("\nNo worries, no units were sent away!\n");
                    }
                }
            }
            "5" | "q" | "Q" | "quit" | "Quit" | "QUIT" => return Actions::Quit,
            "6" | "h" | "H" | "help" | "Help" | "HELP" => print_help(),
            "7" | "stats" | "Stats" | "STATS" | "statistics" | "Statistics" | "STATISTICS" => {
                println!("\n{}\n", player.status(round, game_plan, "during"))
            }
            "8" | "rules" | "Rules" | "RULES" => print_rules(),
            _ => {
                println!(
                    "\nUnknown command! Please, type '6' or 'help' and hit enter to see help.\n"
                )
            }
        }
    }
}

/// Used for specifying the desired units action.
enum UnitAction {
    Conquer(usize, usize),
    Train,
}

/// Function that can either return a unit action,
/// specified by its parameter, or return none
///
/// Returns
/// ---
/// - Some(action): if user decides to specify a unit action
/// - None: if user chose to leave the unit action specification
fn units_action(player: &Player, unit_action: UnitAction) -> Option<Actions> {
    let unit_type: UnitType;

    // auxiliary output variables
    let action: &str;
    let action_past: &str;
    let action_0_units: &str;
    let action_units_counted: String;

    // fill auxiliary variables output strings
    match unit_action {
        UnitAction::Train => {
            action = "train";
            action_past = "trained";
            action_0_units = "train";
            action_units_counted = match player.current_fighters_capacity() {
                // when there is no capacity,
                0 => String::from(
                    "You cannot currently train any units. Consider building a base first.",
                ),
                _ => format!(
                    "You can currently train {} units of type {} *OR* {} units of type {}.",
                    player.train_max_units(UnitType::Archer),
                    UnitType::Archer,
                    player.train_max_units(UnitType::Warrior),
                    UnitType::Warrior,
                ),
            }
        }
        UnitAction::Conquer(_, _) => {
            action = "send to conquer";
            action_past = "sent to conquer";
            action_0_units = "send";
            action_units_counted = match player.has_fighters_available() {
                true => {
                    format!(
                        "You can send {} units of type {} *OR* {} units of type {}.",
                        player.send_max_units(UnitType::Archer),
                        UnitType::Archer,
                        player.send_max_units(UnitType::Warrior),
                        UnitType::Warrior,
                    )
                }
                false => String::from(
                    "Cannot currently send any units. Consider training some units instead.",
                ),
            }
        }
    }

    // get unit type
    loop {
        println!(
            "\nPlease specify which unit type you want to {}:\n{}\n(possible options: 'ARCHER', 'WARRIOR')\n(to quit, type 'QUIT', 'quit' or 'q')\n",
            action, action_units_counted
        );

        // get the line and trim it
        let line = get_line();
        let line = line.trim();

        // obtain information from line
        match line {
            "ARCHER" | "archer" => {
                unit_type = UnitType::Archer;
                break;
            }
            "WARRIOR" | "warrior" => {
                unit_type = UnitType::Warrior;
                break;
            }
            "QUIT" | "Quit" | "Q" | "quit" | "q" => return None,
            _ => {
                println!("\nUnknown unit type, the units will not be {}.\nType 'QUIT', 'quit' or 'q' to change your move.\n", action_past);
            }
        };
    }

    // print choice
    println!("\nUnit type picked: {}\n", unit_type);

    // get unit quantity
    loop {
        println!(
            "\nPlease specify how many troops of type {} you wish to {}:\n",
            unit_type, action
        );

        // get the line and trim it
        let line = get_line();
        let line = line.trim();

        // obtain quantity
        match line.parse::<i32>() {
            // correct quantity passed
            Ok(n) if n > 0 => {
                // return desired action
                match unit_action {
                    UnitAction::Train => return Some(Actions::Train(unit_type, n)),
                    UnitAction::Conquer(x, y) => return Some(Actions::Conquer(x, y, unit_type, n)),
                }
            }
            // 0 units -> incorrect input
            Ok(n) if n == 0 => {
                println!(
                    "\nCannot {} 0 units of type {}!\n",
                    action_0_units, unit_type
                );
            }
            // negative numbers -> incorrect input
            Ok(_) => {
                println!(
                    "\nCannot {} a negative number of units of type {}!\n",
                    action_0_units, unit_type
                );
            }
            // could not parse to a number
            Err(_) => {
                // check whether user wanted to quit the training choice
                match line {
                    "QUIT" | "Quit" | "Q" | "quit" | "q" => return None,
                    _ => println!("\nIncorrect format! Please put a positive number to specify number of units!\n(To quit, type 'QUIT', 'quit' or 'q')\n"),
                }
            }
        }
    }
}
