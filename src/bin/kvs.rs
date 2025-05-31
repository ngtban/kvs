use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
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
