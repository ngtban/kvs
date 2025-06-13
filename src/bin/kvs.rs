use clap::Parser;
use kvs::Commands;
// use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(about = "kvs: a cli-based key-value store", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    // let mut store = KvStore::open("command_log.rkyv");

    match args.command {
        Commands::Get { .. } => {
            panic!("unimplemented")
        }
        Commands::Set { .. } => {
            panic!("unimplemented")
        }
        Commands::Rm { .. } => {
            panic!("unimplemented")
        }
    }
}
