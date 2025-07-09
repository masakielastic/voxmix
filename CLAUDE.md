# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`voxmix` is a Rust CLI tool that generates high-quality speech synthesis files from text using the VOICEVOX engine. It supports both HTTP API and C API modes and enables automatic voice generation and podcast creation from articles and scripts.

## Key Features

- **say subcommand**: Synthesizes specified text into speech with customizable speaker, volume, speed, and pitch
- **render subcommand**: Processes text files with multiple speakers and dialogues into a single audio file
- **Dual API support**: HTTP API (when --host/--port specified) or C API (native mode)

## Required Dependencies

Based on spec.md, the following Rust libraries are essential:

- `clap` - CLI argument and subcommand parsing (with derive feature)
- `serde` - Configuration and struct deserialization
- `reqwest` - HTTP API client for VOICEVOX HTTP API
- `hound` - WAV file reading/writing
- `anyhow` - Error handling
- `tracing` - Logging
- `indicatif` - Progress bars
- `thiserror` - Custom error type derive
- `libvoicevox_core-sys` - VOICEVOX C API bindings (requires wrapper)

## Development Commands

Since this is a new Rust project without existing Cargo.toml:

```bash
# Initialize new Rust project
cargo init

# Build the project
cargo build

# Run with arguments
cargo run -- say --speaker "ずんだもん" --output out.wav "こんにちは"

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy linting
cargo clippy

# Run a specific test
cargo test test_name
```

## Architecture Notes

The CLI should be structured with:
1. Main entry point handling subcommand routing
2. Separate modules for `say` and `render` commands
3. Abstraction layer for HTTP vs C API modes
4. Common utilities for audio processing and speaker management
5. Error handling with custom error types using `thiserror`

## Commit Rules

This project follows Conventional Commits specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Common types:
- `feat`: new feature
- `fix`: bug fix
- `docs`: documentation changes
- `style`: code style changes
- `refactor`: code refactoring
- `test`: adding or updating tests
- `chore`: maintenance tasks

Examples:
- `feat(say): add volume control option`
- `fix(render): handle empty input files`
- `docs: update CLI usage examples`

## Rust Version

Requires Rust 1.70 or later as specified in spec.md.