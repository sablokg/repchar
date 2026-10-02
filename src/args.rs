use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "repchar",
    version = "1.0",
    about = "Microbial Repeat Analyzer
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Window Repeat Analyzer
    Window {
        /// path to the sequence file
        filepath: String,
        /// type of repeat such as 3 or 4 or higher
        repeat: String,
        /// threads for the analysis
        thread: String,
    },
    /// Specific Repeat Analyzer
    SpecificWindow {
        /// path to the sequence file
        filepath: String,
        /// type of repeat such as 3 or 4 or higher
        repeat: String,
        /// threads for the analysis
        thread: String,
    },
    /// Table analyzer
    Table {
        /// path to the sequence file
        filepath: String,
        /// type of repeat such as 3 or 4 or higher
        repeat: String,
        /// threads for the analysis
        thread: String,
    },
    /// Stretch Count analyzer
    Count {
        /// path to the sequence file
        filepath: String,
        /// type of the nucleotide such as A, AT, AG, AGA
        nucleotide: String,
        /// count of the same
        count: String,
    },
}
