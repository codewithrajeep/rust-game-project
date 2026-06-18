# 🦀 Rust Game Project

A collection of simple games and projects built while learning **Rust**. This repository is organized as a **Cargo Workspace**, allowing multiple independent projects to coexist in a single repository.

## Projects

| Project                              | Description                                                    | Status         |
| :----------------------------------- | :------------------------------------------------------------- | :------------- |
| **[Guessing Game](./guessing_game)** | A classic number guessing game using random number generation. | ✅ Completed   |
| **[To-Do List](./todo_list)**        | A command-line interface (CLI) to manage tasks.                | 🚧 In Progress |

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

**3. Build All Projects:**

```bash
cargo build --workspace
```
