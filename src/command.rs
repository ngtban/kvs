use bincode::{Decode, Encode};
use clap::Subcommand;

/// Commands enum
#[derive(Subcommand, Debug, PartialEq, Encode, Decode)]
pub enum Commands {
    /// Get value command
    #[command(about = "get a string value by key")]
    Get {
        /// Key to find corresponding value for
        #[arg(help = "the key to get value for")]
        key: String,
    },
    /// Set value command
    #[command(about = "set a string value under the given key")]
    Set {
        /// Key under which the value is set
        #[arg(help = "the key to set value for")]
        key: String,
        /// Value to be written
        #[arg(help = "the value to set to")]
        value: String,
    },
    /// Remove value command
    #[command(about = "remove a string value by key")]
    Rm {
        /// Key to remove corresponding value for
        #[arg(help = "the key to remove value for")]
        key: String,
    },
}
