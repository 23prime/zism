# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## General agent rules

- When users ask questions, answer them instead of doing the work.

### Shell Rules

- Always use `rm -f` (never bare `rm`)
- Run `git` commands in the current directory (do not use the `-C` option)

## Project Overview

zism (Zellij Interactive Session Manager) is a CLI tool for interactively managing Zellij sessions.
It runs outside of Zellij sessions and provides interactive create, attach, and delete operations.

- Language: Rust (edition 2024)
- No argument parser needed. Run simply as `zism` for a purely interactive CLI

### Architecture

```txt
src/
├── main.rs      # Entry point and main flow
├── zellij.rs    # Zellij command wrapper (parsing, arg building, execution)
└── ui.rs        # Interactive prompts using inquire (Action enum, selection, input)
```

### Key dependencies

- `anyhow` - Error handling
- `inquire` - Interactive prompts

## Development

### Workflow

Follow TDD workflow:

1. Write a list of the test scenarios you want to cover
2. 🔴 [RED] Write failing test: Turn **exactly one** item on the list into an actual, concrete, runnable test
3. 🟢 [GREEN] Write minimal code to pass: Change the code to make the test (& all previous tests) pass (adding items to the list as you discover them)
4. 🔵 [REFACTOR] Improve code quality: Optionally refactor to improve the implementation design
5. Until the list is empty, go back to #2

### Build and check

```sh
mise run rs-check   # clippy + fmt --check + test
mise run rs-fix     # clippy --fix + fmt
mise run rs-build   # cargo build
mise run check      # all checks (markdown, GitHub Actions, etc.)
```

### Testing

```sh
mise run rs-check   # includes cargo test
```

- Separate testable pure logic from external command execution and interactive UI
  - `parse_sessions`, `build_*_args`: Command output parsing and argument construction
  - `available_actions`: Action candidates based on session availability
  - `is_inside_zellij`: Check if running inside a Zellij session
