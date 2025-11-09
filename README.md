# 🦀 Habit Tracker CLI

A minimal, educational command-line habit tracker built with Rust. Track your daily habits with simple commands and store everything in a local JSON file—no database required!

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## 📚 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Technical Deep Dive](#technical-deep-dive)
- [Learning Objectives](#learning-objectives)
- [Example Workflow](#example-workflow)
- [Data Storage](#data-storage)
- [Contributing](#contributing)

---

## 🎯 Overview

This project is designed as an **educational resource** for learning Rust fundamentals through a practical, real-world application. It demonstrates core concepts like:

- **File I/O** operations
- **JSON serialization/deserialization** with Serde
- **CLI argument parsing** with Clap
- **Data structures** (HashMap, Vec)
- **Error handling** with Result types
- **Date manipulation** with Chrono

Perfect for beginners looking to understand how different Rust crates work together, or as a portfolio piece showcasing clean, well-documented code.

---

## ✨ Features

- ✅ **Add habits** to track
- 🎉 **Mark habits as done** for the current day
- 📊 **View statistics** including total completions and current streaks
- 📝 **List all habits** you're tracking
- 💾 **Persistent storage** in human-readable JSON
- 🚀 **Zero external dependencies** for runtime (standalone binary)
- 📖 **Extensively commented** code for learning

---

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later recommended)
- Cargo (comes with Rust)

### Setup

1. **Clone or create the project:**
   ```bash
   cargo new habit_tracker
   cd habit_tracker
   ```

2. **Update `Cargo.toml` with dependencies:**
   ```toml
   [dependencies]
   clap = { version = "4.4", features = ["derive"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   chrono = "0.4"
   ```

3. **Replace `src/main.rs`** with the habit tracker code

4. **Build the project:**
   ```bash
   cargo build --release
   ```

5. **Run the binary:**
   ```bash
   # Run directly with cargo
   cargo run -- <command>
   
   # Or use the compiled binary
   ./target/release/habit_tracker <command>
   ```

---

## 🎮 Usage

### Commands

#### **Add a new habit**
```bash
cargo run -- add <habit-name>
```
Creates a new habit to track.

**Example:**
```bash
cargo run -- add workout
# Output: ✅ Added habit: 'workout'
```

#### **Mark a habit as done**
```bash
cargo run -- done <habit-name>
```
Records a completion for today. Prevents duplicate entries for the same day.

**Example:**
```bash
cargo run -- done workout
# Output: 🎉 Marked 'workout' as done for today!
```

#### **View statistics**
```bash
cargo run -- stats
```
Displays all habits with their total completions and current streak.

**Example output:**
```
📊 Habit Statistics

Habit                Total Done      Current Streak
--------------------------------------------------
workout              7               3 days
meditation           12              5 days
reading              4               1 days
```

#### **List all habits**
```bash
cargo run -- list
```
Shows all habits you're currently tracking.

**Example output:**
```
📝 Your Habits:

  • workout
  • meditation
  • reading
```

---

## 📂 Project Structure

```
habit_tracker/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project dependencies
├── Cargo.lock           # Dependency lock file
├── habits.json          # Data storage (created automatically)
└── README.md            # This file
```

### Key Files

- **`main.rs`**: Contains all application logic
- **`habits.json`**: Stores habit data in JSON format
- **`Cargo.toml`**: Defines project metadata and dependencies

---

## 🔬 Technical Deep Dive

### Architecture Overview

The application follows a simple architecture:

```
CLI Input → Parser (Clap) → HabitTracker → File I/O (JSON)
```

### Core Components

#### **1. CLI Parsing with Clap**

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Done { name: String },
    Stats,
    List,
}
```

**How it works:**
- `#[derive(Parser)]` automatically generates argument parsing code
- Clap handles help messages, validation, and error messages
- Subcommands are defined as enum variants

#### **2. Data Structures**

```rust
struct HabitTracker {
    habits: HashMap<String, Habit>,
}

struct Habit {
    name: String,
    completions: Vec<String>,
}
```

**Design decisions:**
- **HashMap**: Provides O(1) lookup by habit name
- **Vec<String>**: Stores completion dates as strings (format: "YYYY-MM-DD")
- Simple, flat structure that maps cleanly to JSON

#### **3. Serialization with Serde**

```rust
#[derive(Serialize, Deserialize)]
struct HabitTracker { /* ... */ }
```

**How it works:**
- `#[derive(Serialize, Deserialize)]` generates conversion code
- `serde_json::to_string_pretty()` converts Rust → JSON
- `serde_json::from_str()` converts JSON → Rust
- Zero manual parsing required!

#### **4. File I/O**

```rust
// Load from file
let data = fs::read_to_string(DATA_FILE)?;
let tracker: HabitTracker = serde_json::from_str(&data)