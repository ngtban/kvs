use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
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
        Some(Commands::Get { key: _key }) => { panic!("unimplemented") },
        Some(Commands::Set { key: _key, value: _value }) => { panic!("unimplemented") },
        Some(Commands::Rm { key: _key }) => { panic!("unimplemented") },
        None => { println!("no subcommand given") },
    }
}
