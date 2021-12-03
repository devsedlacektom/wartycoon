use super::sleep_intervals::{game_sleep_second, game_sleep_two_seconds};
use super::types::{board::GamePlan, player::Player};

// default game prints
const GAME_INITIAL_GREETING: &str = "Welcome to WarTycoon! An interactive command line game.\nWe hope you have a great time playing with us!\n";
const GAME_START: &str = "The game is starting! Fasten your seatbelts and prepare for adventure!\n";

/// Notify a player that it's their turn
///
/// Params
/// ---
/// - player: player reference, used for displaying their name
/// - round: used for displaying which round it is
pub fn notify_players_turn(player: &Player, round: usize) {
    let line = "─".repeat(78);
    println!(
        "╭{}╮\n│{:^78}│\n╰{}╯\n\n",
        &line,
        format!("It's {}'s turn for round {}!", player.nick, &round),
        &line,
    )
}

/// Notify user the game has started
pub fn print_game_start() {
    println!("{}", GAME_START);
    game_sleep_second();
}

/// Greet user before the start of the game
pub fn print_greeting() {
    println!("{}", GAME_INITIAL_GREETING);
    game_sleep_second();
}

/// Print help -> which actions can user invoke
pub fn print_help() {
    println!("\nROUND CONTROLS:\n-'1' or 'build', 'Build', 'BUILD' to build a base\n\n-'2' or 'harvest', 'Harvest', 'HARVEST' to harvest resources\n\n-'3' or 'train', 'Train', 'TRAIN' to train units,\n  hit enter and then type unit type (for example 'ARCHER')\n  hit enter and specify the number of units you wish to train\n\n-'4' or 'conquer', 'Conquer', 'CONQUER' to send troops to conquer a field,\n  then hit enter and specify type (same as in train),\n  hit enter and put a desired number of troops\n\n-'5' or 'q', 'Q', 'quit', 'Quit', 'QUIT' to quit the game\n\n-'6' or 'h', 'H', 'help', 'Help', 'HELP' to display this help\n\n-'7' or 'stats', 'Stats', 'STATS', 'statistics', 'Statistics', 'STATISTICS'\n  to display current player's statistics\n\n-'8' or 'rules', 'Rules', 'RULES' to display game rules.\n");
}

/// Print the result of a game round, along with player's status
///
/// Params
/// ---
/// - notification: string (slice) containing text result of a game action
/// - player: reference of a player that's currently playing
/// - game_plan: reference to the game plan (to obtain how many units a player sent to the field)
/// - round: which game round it is
/// - status_at_the_end: whether to print player's status at the end
pub fn print_round_action(
    notification: &str,
    player: &Player,
    game_plan: &GamePlan,
    round: usize,
    status_at_the_end: bool,
) {
    // straight 78 character long line
    let line_smooth = "═".repeat(78);

    // format string to return a nicely formatted table
    println!(
        "╔{}╗\n║{:^78}║\n╠{}╣\n{}\n╚{}╝\n",
        &line_smooth,
        format!("{}'s action info for round {}:", player.nick, round),
        &line_smooth,
        notification,
        &line_smooth,
    );

    game_sleep_second();

    // display the status at the end as well
    if status_at_the_end {
        println!("{}\n\n", player.status(round, game_plan, "after"));
        game_sleep_two_seconds();
    }
}

/// Print game rules
pub fn print_rules() {
    println!("\n- Harvesting gives player 200 units of wood and 120 units of gold.\n- It is necessary to build a base in order to train units.\n- To build a base, you need 220 units of wood and 100 units of gold\n- Base has a capacity of 200 units. To be able to have more than 200 units at your disposal, you have to build another base.\n- There are two types of units, Archers and Warriors.\n- It costs 10 units of gold to train one Archer.\n- It costs 10 units of wood and 5 units of gold to train one Warrior.\n- Archers are a bit stronger in the field than Warriors. (1.9 strength vs 1.2 strength)\n- You can send troops to conquer a piece of land, your opponent will probably do the same.\n- Player with strongest force on a certain field will be considered the conqueror of that field.\n- At the end of the game, the fields are evaluated and the person with most conquered fields wins.\n- If there are equal forces on the field at the end of the game, it is NOT won.\n- The DEFAULT version of the game only includes one field. Custom game mode may be coming in a future patch.\n- The DEFAULT version of the game only allows 2 players. Custom game modes might be implemented in the next patch.\n- You can decide to quit the game at any round. Please, know that the round will continue for other players.\n");
}
