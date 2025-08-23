# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rs-iching is a Rust application that generates random I Ching divinations using the traditional three-coin method. The project simulates the ancient Chinese divination practice by:

1. Generating hexagrams with six lines (each line determined by three coin tosses)
2. Identifying changing lines that transform the present into a future hexagram
3. Looking up hexagram numbers using the King Wen sequence
4. Displaying the divination results with trigram names

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run`
- **Build (release)**: `cargo build --release`
- **Run (release)**: `cargo run --release`
- **Check code**: `cargo check`
- **Run tests**: `cargo test`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`

## Architecture

### Core Components

- **Line enum**: Represents I Ching lines with four states (StaticYin, StaticYang, ChangingYin, ChangingYang)
- **Hexagram struct**: Contains six lines, calculates King Wen numbers, and manages trigram relationships
- **Divination struct**: Orchestrates the complete divination process from present to future hexagrams

### Key Implementation Details

- Lines are generated using three-coin method where heads=3, tails=2, with sums mapping to line types (6→ChangingYin, 7→StaticYang, 8→StaticYin, 9→ChangingYang)
- Hexagram numbers calculated by converting line patterns to binary then mapping to King Wen sequence via lookup table
- Lines stored in generation order (index 0 = bottom line), but displayed in traditional order (top to bottom)
- Future hexagrams created by changing all moving lines, or None if no changing lines exist

### Data Files

- `data/wade_giles.csv`: Translation data (currently minimal, intended for hexagram names, images, judgements, and changing line interpretations)

### Dependencies

- `rand = "0.9.1"`: For random number generation in coin tosses