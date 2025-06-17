use clap::Parser;
use kvs::KvStore;
use kvs::{Commands, KvsError, Result};
use std::env::current_dir;
use std::process;

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(about = "kvs: a cli-based key-value store", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut store = match KvStore::open(current_dir()?) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

    match args.command {
        Commands::Get { key } => match store.get(key) {
            Ok(Some(value)) => {
                print!("{}", value);
                Ok(())
            }
            Ok(None) => {
                print!("Key not found");
                Ok(())
            }
            Err(e) => Err(e),
        },
        Commands::Set { key, value } => match store.set(key, value) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        },
        Commands::Rm { key } => match store.remove(key) {
            Ok(_) => Ok(()),
            Err(KvsError::KeyNotFound) => {
                print!("Key not found");
                process::exit(1);
            }
            Err(e) => Err(e),
        },
    }
}
