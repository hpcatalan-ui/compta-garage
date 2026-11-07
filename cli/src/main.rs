//! Garage Compta - CLI
//!
//! Command-line interface for Garage Compta.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "garage-compta")]
#[command(about = "Garage Compta - Accounting software for garages", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server
    Serve {
        /// Port to serve on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Initialize the database
    Init {
        /// Database path
        #[arg(short, long, default_value = "./data/garage.db")]
        database: String,
    },
    /// Run database migrations
    Migrate {
        /// Database path
        #[arg(short, long, default_value = "./data/garage.db")]
        database: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Serve { port } => {
            println!("Starting server on port {}", port);
            // TODO: Implement server
        }
        Commands::Init { database } => {
            println!("Initializing database at {}", database);
            // TODO: Implement init
        }
        Commands::Migrate { database } => {
            println!("Running migrations on database at {}", database);
            // TODO: Implement migrations
        }
    }
    
    Ok(())
}
