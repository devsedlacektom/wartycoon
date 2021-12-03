// This game is intentionally written more complicated than necessary, allowing
// it to be upgraded (f.e. custom battlefield size mode, more players mode etc.)
// therefore the code is sometimes way more complicated than it needs to.
//
// It IS intentional, please keep that in mind.

mod game;

// Player structure
use game::types::player::Player;

// use public game interface
use game::{create_players, evaluate_game, generate_game_plan, get_number_of_rounds, play_round};

// use interval for round sleep
use game::sleep_intervals::game_round_sleep;

// use game notifications
use game::notifications::{print_game_start, print_greeting};

// default number of players
const DEFAULT_NUM_PLAYERS: usize = 2;

fn main() {
    // print greeting
    print_greeting();

    // create a game plan
    let mut game_plan = generate_game_plan(1, 1);

    // create a specified number of players
    // also could be implemented for more than two players,
    // this is a setup for implementing it later
    // if I choose to do so
    let mut players: Vec<Player> = create_players(DEFAULT_NUM_PLAYERS);

    // obtain number of rounds to play
    let rounds: usize = get_number_of_rounds();
    let number_of_players = players.len();

    // print successful start of the game
    print_game_start();

    // play desired number of rounds
    for current_round in 1..rounds + 1 {
        let mut continue_game = true;

        // every player gets to play each round
        for (player_number, player) in players.iter_mut().enumerate() {
            // if a player decides to quit, this gets set to false
            let player_exit = play_round(player, &mut game_plan, current_round);

            // check whether to play another round
            continue_game &= player_exit;

            // next player announcement only appears if another round is to be played
            if player_number != number_of_players - 1 {
                println!("Next player will begin shortly.\n\n");
            }

            game_round_sleep();
        }

        // after the round is over, if someone requested for the end of the game, it ends
        if !continue_game {
            break;
        }
    }

    // evaluate the game
    evaluate_game(&game_plan);
}
