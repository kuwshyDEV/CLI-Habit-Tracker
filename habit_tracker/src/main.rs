use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// This is the file where we'll store all our habits
const DATA_FILE: &str = "habits.json";

/// A simple command-line habit tracker
/// 
/// This program helps you track daily habits by storing them in a local JSON file.
#[derive(Parser)]
#[command(name = "Habit Tracker")]
#[command(about = "Track your daily habits", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// All the commands our habit tracker supports
#[derive(Subcommand)]
enum Commands {
    /// Add a new habit to track
    /// 
    /// Example: habit add workout
    Add {
        /// Name of the habit (e.g., "workout", "reading", "meditation")
        name: String,
    },
    
    /// Mark a habit as done for today
    /// 
    /// Example: habit done workout
    Done {
        /// Name of the habit to mark complete
        name: String,
    },
    
    /// Show statistics for all habits
    /// 
    /// Displays total completions and current streak for each habit
    Stats,
    
    /// List all habits
    List,
}

/// Represents a single habit and its completion history
/// 
/// The #[derive] attributes automatically implement common traits:
/// - Serialize/Deserialize: Convert to/from JSON
/// - Debug: Allow printing with {:?}
/// - Clone: Create copies of the struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Habit {
    /// Name of the habit (e.g., "workout")
    name: String,
    
    /// List of dates when this habit was completed (format: "YYYY-MM-DD")
    completions: Vec<String>,
}

impl Habit {
    /// Create a new habit with no completions yet
    fn new(name: String) -> Self {
        Habit {
            name,
            completions: Vec::new(),
        }
    }
    
    /// Calculate the current streak (consecutive days completed)
    /// 
    /// Returns the number of consecutive days this habit has been done,
    /// counting backwards from today.
    fn calculate_streak(&self) -> usize {
        if self.completions.is_empty() {
            return 0;
        }
        
        // Sort completions in reverse chronological order (newest first)
        let mut sorted_completions = self.completions.clone();
        sorted_completions.sort();
        sorted_completions.reverse();
        
        let mut streak = 0;
        let mut current_date = chrono::Local::now().date_naive();
        
        // Count consecutive days going backwards from today
        for completion in sorted_completions {
            let completion_date = chrono::NaiveDate::parse_from_str(&completion, "%Y-%m-%d");
            
            if let Ok(comp_date) = completion_date {
                // Check if this completion matches our expected date
                if comp_date == current_date {
                    streak += 1;
                    // Move to the previous day
                    current_date = current_date.pred_opt().unwrap();
                } else {
                    // Streak is broken
                    break;
                }
            }
        }
        
        streak
    }
}

/// The main data structure holding all habits
/// 
/// We use a HashMap for O(1) lookup by habit name
#[derive(Serialize, Deserialize, Debug)]
struct HabitTracker {
    /// Maps habit names to Habit structs
    /// Example: "workout" -> Habit { name: "workout", completions: [...] }
    habits: HashMap<String, Habit>,
}

impl HabitTracker {
    /// Create a new empty habit tracker
    fn new() -> Self {
        HabitTracker {
            habits: HashMap::new(),
        }
    }
    
    /// Load habits from the JSON file
    /// 
    /// If the file doesn't exist, return a new empty tracker.
    /// This is called every time we run a command.
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Check if the file exists
        if !Path::new(DATA_FILE).exists() {
            // File doesn't exist yet - that's okay, return empty tracker
            return Ok(HabitTracker::new());
        }
        
        // Read the entire file into a String
        let data = fs::read_to_string(DATA_FILE)?;
        
        // Parse the JSON string into our HabitTracker struct
        // serde_json does all the heavy lifting here
        let tracker: HabitTracker = serde_json::from_str(&data)?;
        
        Ok(tracker)
    }
    
    /// Save habits to the JSON file
    /// 
    /// This writes the entire HashMap to disk as formatted JSON.
    /// Called after any command that modifies the data.
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Convert our struct to a pretty-printed JSON string
        // The "pretty" formatting makes it human-readable with indentation
        let json = serde_json::to_string_pretty(&self)?;
        
        // Write the JSON string to the file (creates or overwrites)
        fs::write(DATA_FILE, json)?;
        
        Ok(())
    }
    
    /// Add a new habit to track
    fn add_habit(&mut self, name: String) {
        // Check if habit already exists
        if self.habits.contains_key(&name) {
            println!("⚠️  Habit '{}' already exists!", name);
            return;
        }
        
        // Create new habit and add to HashMap
        let habit = Habit::new(name.clone());
        self.habits.insert(name.clone(), habit);
        
        println!("✅ Added habit: '{}'", name);
    }
    
    /// Mark a habit as completed for today
    fn mark_done(&mut self, name: String) {
        // Try to get the habit from the HashMap
        match self.habits.get_mut(&name) {
            Some(habit) => {
                // Get today's date
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                
                // Check if already marked done today
                if habit.completions.contains(&today) {
                    println!("ℹ️  You already completed '{}' today!", name);
                    return;
                }
                
                // Add today to the completions list
                habit.completions.push(today);
                println!("🎉 Marked '{}' as done for today!", name);
            }
            None => {
                println!("❌ Habit '{}' not found. Add it first with 'habit add {}'", name, name);
            }
        }
    }
    
    /// Display statistics for all habits
    fn show_stats(&self) {
        if self.habits.is_empty() {
            println!("No habits tracked yet. Add one with 'habit add <name>'");
            return;
        }
        
        println!("\n📊 Habit Statistics\n");
        println!("{:<20} {:<15} {:<15}", "Habit", "Total Done", "Current Streak");
        println!("{}", "-".repeat(50));
        
        // Iterate through all habits
        for (name, habit) in &self.habits {
            let total = habit.completions.len();
            let streak = habit.calculate_streak();
            
            println!("{:<20} {:<15} {:<15}", name, total, format!("{} days", streak));
        }
        
        println!();
    }
    
    /// List all tracked habits
    fn list_habits(&self) {
        if self.habits.is_empty() {
            println!("No habits tracked yet. Add one with 'habit add <name>'");
            return;
        }
        
        println!("\n📝 Your Habits:\n");
        for name in self.habits.keys() {
            println!("  • {}", name);
        }
        println!();
    }
}

fn main() {
    // Parse command-line arguments using clap
    let cli = Cli::parse();
    
    // Load existing habits from file (or create new tracker if file doesn't exist)
    let mut tracker = HabitTracker::load().unwrap_or_else(|err| {
        eprintln!("Error loading habits: {}", err);
        HabitTracker::new()
    });
    
    // Execute the appropriate command
    match cli.command {
        Commands::Add { name } => {
            tracker.add_habit(name);
            // Save changes to disk
            if let Err(e) = tracker.save() {
                eprintln!("Error saving habits: {}", e);
            }
        }
        Commands::Done { name } => {
            tracker.mark_done(name);
            // Save changes to disk
            if let Err(e) = tracker.save() {
                eprintln!("Error saving habits: {}", e);
            }
        }
        Commands::Stats => {
            // No need to save for read-only operations
            tracker.show_stats();
        }
        Commands::List => {
            // No need to save for read-only operations
            tracker.list_habits();
        }
    }
}