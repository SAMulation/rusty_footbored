# FootBored Game in Rust

(Last updated: 6/7/2023 at 00:57 PST)
This is a Rust implementation of the football "bored" game, FootBored. This project serves as a practice ground for Rust programming, with the long-term goal of potentially acting as the backend for the web-based version of the game. 

## Background

The original game of FootBored is written in JavaScript and you can play the game at [footbored.com](https://footbored.com). The source code of the original game can be found at its [GitHub repository](https://github.com/SAMulation/fbg-5.1).

This Rust reimagination of the game is currently a work in progress.

## Game Structure

FootBored currently consists of the following main components (more to come):

- `Player`: Each player is assigned a set of `PlayCards` which represent different types of plays that can be executed in the game.
- `Game`: The main engine of the game which manages the game state, including the `Player`s, the `PlayCards`, and the overall progression of the game.
- `Play`: Represents the different types of plays available in the game.

## Project TODO List:

### Stage 1: Establish Basic Game Mechanics
- Implement game start conditions:
  - Start game at 25 with Player 1 (p1) on offense
  - Include a "distance stick" mechanic
- Create scoring mechanisms:
  - Implement a safety score (2 points to defense, change possession)
  - Implement touchdown scoring (6 points to offense, change possession)
- Develop basic game flow mechanics:
  - Set up `change_poss()` function to swap possession and reset ball position to 25
  - Implement a rudimentary system for status updates after each play

### Stage 2: Add Game Duration and Progression
- Implement "downs" and "togo" mechanics
- Introduce a simple timing system:
  - Add quarters to the game structure
  - Include a clock decrement mechanic
  - Implement quarter changes
  - End game at the 4th quarter (can end in a tie)

### Stage 3: Customize Game Settings
- Add introductory player inputs:
  - Player and team names
  - Quarter length

### Stage 4: Enhance Game Mechanics
- (Deferred) Enrich gameplay by adding more complex mechanics:
  - Play selection
  - Special plays
  - Timeouts
  - Kicking

Note: Stage 4 mechanics are currently being deferred for future consideration.

## Usage

To run the game, you need to have Rust installed. Once Rust is installed, you can use Cargo to run the game:

```bash
cargo run
```

## Contributing
This is a pet project and its progress depends heavily on available free time. Contributions are welcome, though! Feel free to open issues or pull requests.

## License
This project is open-source and available under the [MIT License](https://www.mit.edu/~amini/LICENSE.md).