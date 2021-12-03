// There is a possibility, that a custom game mode will be implemented.
// This game mode would allow for multiple fields also printing the map
// In a nice and formatted way.

mod player_action;
pub mod sleep_intervals;
pub mod types;
mod user_input;

use player_action::{confirm_action, get_player_action};

// input handling
use user_input::get_line;

// game notifications
pub mod notifications;
use notifications::{notify_players_turn, print_round_action};

// sleep intervals
use sleep_intervals::{game_sleep_half_second, game_sleep_second};

use types::{actions::Actions, board::GamePlan, player::Player};

// **********************************************************
// *                                                        *
// *                                                        *
// *                 PUBLIC GAME INTERFACE                  *
// *                                                        *
// *                                                        *
// **********************************************************

/// Create specified number of players
///
/// Params
/// ---
/// - num_of_players: specified number of players to create
///
/// Returns
/// ---
/// - vector of created players
pub fn create_players(num_of_players: usize) -> Vec<Player> {
    // new vector of players
    let mut players: Vec<Player> = Vec::new();

    // to create a desired number of players
    (0..num_of_players).into_iter().for_each(|n| {
        // loop here to be able to correct mistakes
        loop {
            // ask player to input the name
            println!("\nPlease put in a name of PLAYER {}:", &n + 1);
            // get and trim the line
            let line = get_line();
            let line = line.trim();

            // try to create the player
            match create_player(line, &players) {
                // no duplicates, player has been created.
                Ok(player) => {
                    players.push(player);
                    println!("\nPlayer {} has been successfully created!", line);
                    // stop the loop
                    game_sleep_half_second();
                    break;
                }
                // the player exists, loops again
                Err(error) => {
                    println!("\nERROR: {}\n", error);
                    game_sleep_half_second();
                }
            }
        }
    });

    // return players
    players
}

/// Evaluate the game and print the winner.
///
/// Params
/// ---
/// - game_plan: reference to the game plan
pub fn evaluate_game(game_plan: &GamePlan) {
    game_plan.evaluate();
}

/// Generate game plan with desired width and height
///
/// Params
/// ---
/// - width: width of the game plan
/// - height: height of the game plan
///
/// Returns
/// ---
/// - GamePlan: fresh instance of a game plan
pub fn generate_game_plan(width: usize, height: usize) -> GamePlan {
    // create a new game plan
    let plan = GamePlan::new(width, height);
    // obtain plan's dimensions
    let dimensions = plan.get_dimensions();

    game_sleep_second();
    // print plan creation
    println!(
        "\nGame plan with dimensions {} has been created.\n",
        dimensions
    );

    // return the plan
    plan
}

/// Get the number of rounds from player
///
/// Returns
/// ---
/// - usize: number of rounds to be played
pub fn get_number_of_rounds() -> usize {
    // input loop in case of a wrong input
    loop {
        println!("\nPlease specify number of rounds you wish to play:\n(put a positive whole number above 9)");
        // get the line & trim it
        let line = get_line();
        let line = line.trim();

        // parse the line as a number (positive number)
        match line.parse::<usize>() {
            Ok(result) if result >= 10 => {
                // correct format and plausible number of rounds
                println!("\nNumber of game rounds will be: {}\n", &result);
                return result;
            }
            Ok(result) => println!(  // correct format but fewer rounds than needed
                "\nCannot play a game with {} rounds! Match too short\n",
                &result
            ),
            Err(..) => {
                println!(  // incorrect format (either a negative number or could not parse the input)
                "\nIncorrect format: {}. Please put a positive whole number above 9!\n",
                line
            )
            }
        };
    }
}

/// Play a round for a player
/// Returns an information if the player chose to finish the game after the end of the round
///
/// Params
/// ---
/// - player: mutable reference to player who currently plays
///           their turn, to be able to modify their internal state
/// - game_plan: mutable reference to be able to affect a game plan (conquer a field)
/// - current_round: number for displaying which round it is
///
/// Returns
/// ---
/// - false: if player chose to quit the game
/// - true: otherwise (after player correctly played their turn)
pub fn play_round(player: &mut Player, game_plan: &mut GamePlan, current_round: usize) -> bool {
    // notify player it's their turn
    notify_players_turn(player, current_round);

    // print the user's status
    player.status(current_round, game_plan, "at the start of");

    // loop for action confirmation and checking whether the operation was successful
    loop {
        let action = get_player_action(player, game_plan, current_round);

        // if the action was not confirmed, continue with choosing an action
        // == starting the loop again
        if !confirm_action(&action) {
            continue;
        }

        // check if the user wants to end the game
        if action == Actions::Quit {
            return false;
        }

        match player.perform_action(action, game_plan) {
            // action was a success
            Ok(notification) => {
                // print action confirmation & user status afterwards
                print_round_action(&notification, player, game_plan, current_round, true);
                game_sleep_half_second();
                return true;
            }
            // action was a failure
            Err(notification) => {
                // don't print user status after action rejection
                print_round_action(&notification, player, game_plan, current_round, false);
                game_sleep_half_second();
            }
        };
    }
}

// **********************************************************
// *                                                        *
// *                                                        *
// *                   AUXILIARY FUNCTION                   *
// *                                                        *
// *                                                        *
// **********************************************************

/// Create a player with specified nick
///
/// Params
/// ---
/// - player_nick: desired nick of our new player
/// - players: vector of existing players of this game
///
/// Returns
/// ---
/// - Ok(player) if the player could be created (i.e. no other player has the same nick)
/// - Err(string) containing details why the player could not be created
fn create_player(player_nick: &str, players: &[Player]) -> Result<Player, String> {
    // find whether there is a player which has the same nick
    let player_exists: Option<&Player> = players.iter().find(|player| player.nick == player_nick);

    // if there is a player with the same name, then an error is raised
    if player_exists.is_some() {
        return Err("Player with this name already exists in the system!".into());
    }

    // player could be created!
    Ok(Player::new(player_nick))
}
