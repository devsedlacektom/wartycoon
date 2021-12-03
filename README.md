# WarTycoon

A simple command line game which features two players that try to outplay each other. This is an assignment for a subject called "Programming in Rust" @FI MUNI.

## Run

```bash
# firstly, you have to build this project. That requires having cargo installed in your system
cd ./wartycoon
cargo build --release

# now you could run this game with cargo, or just run the executable located in the target folder

# cargo version
cargo run --release

cd ./target/release
./wartycoon
```

## Rules

- The goal of the game is to conquer a battlefield.
- Harvesting gives the player 200 units of wood and 120 units of gold.
- It is necessary to build a base in order to train units.
- To build a base, the player need 220 units of wood and 100 units of gold.
- Base has a capacity of 200 units. To be able to have more than 200 units at their disposal, players have to build another base.
- There are two types of units, Archers and Warriors.
- It costs 10 units of gold to train one Archer.
- It costs 10 units of wood and 5 units of gold to train one Warrior.
- Archers are slightly stronger in the field than Warriors. (1.9 strength ratio vs 1.2 strength ratio)
- The player can send out troops to conquer a piece of land.
- Player with strongest force on a certain field will be considered the conqueror of that field at the end of the game.
- At the end of the game, the fields are evaluated and the person with most conquered fields wins.
- If there are equally strong troops on the field at the end of the game, the result is a DRAW.
- The DEFAULT version of the game only includes one battlefield. Custom game mode may be coming in a future patch.
- The DEFAULT version of the game only allows 2 players. Custom game modes might be implemented in the next patch.
- Player can decide to quit the game at any round. Please, know that the round will continue for other players.
