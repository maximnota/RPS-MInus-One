# RPS Minus One 

The AI analyzes the player's past choices and selects moves intelligently—always preferring wins, then ties, then losses.

---

## Features

- **Smart AI:** Scores potential moves against the player's history.
- **Weighted Randomness:** AI prefers higher-scoring moves but keeps some unpredictability.
- **Loss Avoidance:** If winning is impossible, AI will choose a tie over a loss.
- **Rust Implementation:** Lightweight, efficient, and easy to integrate into other Rust projects.

---

## How It Works

The AI chooses a move using the following process:

1. **Score each AI choice:** Compare against all player moves using `rps_result`.
2. **Categorize moves:**
   - **Win:** AI beats the player.
   - **Tie:** AI draws.
   - **Loss:** AI loses.
3. **Pick the best option:**
   - Prefer winning moves.
   - If no wins are available, pick a tie.
   - Only pick a loss if no wins or ties exist.
