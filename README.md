# 🦀 Rust Game Project
A collection of simple games and projects built while learning **Rust**. This repository is organized as a **Cargo Workspace**, allowing multiple independent projects to coexist in a single repository.

## Projects
| Project                              | Description                                                    | Status         |
| :----------------------------------- | :------------------------------------------------------------- | :------------- |
| **[Guessing Game](./guessing_game)** | A classic number guessing game using random number generation. | ✅ Completed   |
| **[To-Do List](./todo_list)**        | A command-line interface (CLI) to manage tasks.                | ✅ Completed   |
| **[Quiz Game](./quiz_game)**         | A CLI quiz game using structs, vectors, and pattern matching.  | ✅ Completed   |

## Getting Started

### Prerequisites
- Install Rust via [rustup](https://rustup.rs/).
- Ensure you have the latest stable version: `rustc --version`.

### How to Run
Since this is a workspace, you need to specify which project to run using the `-p` flag.

**1. Run the Guessing Game:**
```bash
cargo run -p guessing_game
```
**2. Run the To-Do List:**
```bash
cargo run -p todo_list
```
**3. Run the Quiz Game:**
```bash
cargo run -p quiz_game
```
**4. Build All Projects:**
```bash
cargo build --workspace
```

## Learning Journey
This repository documents my progress through "The Rust Programming Language" book and other learning resources.

- **Concepts Covered**: Ownership, Borrowing, Structs, Enums, Pattern Matching, Error Handling, Cargo Workspaces.
- **Crates Used**: `rand` (for random number generation).

### Structure
- `/guessing_game`: Classic number guessing game, built following the official Rust book (Chapter 2).
- `/todo_list`: CLI to-do list built independently using vectors and pattern matching.
- `/quiz_game`: Chapter 2 capstone — a CLI quiz game using structs to group questions, options, and answers.
- `Cargo.toml`: Root workspace configuration file.

### License
This project is open-source and available under the MIT License.