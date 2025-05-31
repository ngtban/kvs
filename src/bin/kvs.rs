use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(about = "command line interface for a key-value store", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "get a string value by key")]
    Get {
        #[arg(help = "the key to get value for")]
        key: String,
    },
    #[command(about = "set a string value under the given key")]
    Set {
        #[arg(help = "the key to set value for")]
        key: String,
        #[arg(help = "the value to set to")]
        value: String,
    },
    #[command(about = "remove a string value by key")]
    Rm {
        #[arg(help = "the key to remove value for")]
        key: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Get { key: _key } => {
            panic!("unimplemented")
        }
        Commands::Set {
            key: _key,
            value: _value,
        } => {
            panic!("unimplemented")
        }
        Commands::Rm { key: _key } => {
            panic!("unimplemented")
        }
    }
}
