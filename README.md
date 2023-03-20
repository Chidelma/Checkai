## Rust Checkers AI Program
This program is a game of checkers implemented in Rust, with an AI that uses the alpha-beta pruning min-max algorithm to determine the next move. It also includes a heuristic scoring system to evaluate the best move, and caches the moves to save time. After every game, the moves are converted into matrices (datapoints) for training using Keras in Python.

Prerequisites
To run this program, you will need Rust and Python 3 installed on your machine.

Installation
To install the program, clone the repository using Git:

```
git clone https://github.com/chidelma/checkai.git
```

## Usage
To play the game, navigate to the root directory of the project and run the following command:

```
cargo run
```

This will start the game and the AI will play against itself.

AI Algorithm
The AI uses the alpha-beta pruning min-max algorithm to determine the next move. This algorithm searches the game tree by exploring each possible move and its potential outcomes. It uses a heuristic scoring system to evaluate the best move based on the current state of the game board.

Caching
The program caches the moves to save time during future games. This means that if the AI encounters a game board state that it has already evaluated, it will retrieve the cached result instead of re-evaluating it.

Training
After every game, the moves are converted into matrices (datapoints) for training using Keras in Python. This allows the AI to improve its performance over time by learning from its past experiences.

Conclusion
This program provides a fun and challenging game of checkers, while also demonstrating the use of the alpha-beta pruning min-max algorithm with a heuristic scoring system. The caching and training features make the AI more efficient and effective, and provide a good foundation for future improvements.
