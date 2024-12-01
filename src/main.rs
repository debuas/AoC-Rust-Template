use crate::days::benchmark::BenchMarkSetting;
use crate::days::run_day;
use clap::{Parser, Subcommand};
use tracing::info;
use crate::scaffold::scaffold;

mod days;
mod scaffold;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Command
}
#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    /// Runs AOC day
    Day {
        day: u8
    },
    #[clap(arg_required_else_help = true)]
    /// Scaffolds a Day if it doesn't exit
    Scaffold {
        day: u8
    },
    #[clap(arg_required_else_help = true)]
    /// Runs Benchmark for AOC days
    Benchmark {
        #[command(subcommand)]
        field: BenchmarkTypes,
        /// Updates the Benchmark entry in README.md
        ///
        /// Uses <!--- BENCHMARK ---> and <!--- BENCHMARK END --->
        /// as delimiters for replacing the table in README.md
        #[arg(short,long,default_value = "false")]
        update: bool,
    }

}


#[derive(Debug,Subcommand)]
enum BenchmarkTypes{
    #[clap(arg_required_else_help = true)]
    Day {
        day : u8
    },
    #[clap(arg_required_else_help = true)]
    UpToDay {

        till:  u8 },
    All

}



fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Day { day } => {info!("Running day {day}") ;run_day(format!("Day{}", day).as_str());}
        Command::Benchmark { field , update } => {
            match field {
                BenchmarkTypes::Day { day } => {info!("Running Bechmarks for day {day}"); BenchMarkSetting::Day(day).run_benchmark()}
                BenchmarkTypes::UpToDay { till } => { info!("Running Bechmarks till day {till}"); BenchMarkSetting::DayRange { min: 1, max: till }.run_benchmark()}
                BenchmarkTypes::All => {info!("Running all Benchmarks") ;BenchMarkSetting::All.run_benchmark()}
            }
            if update {
                info!("Updating Benchmark in Readme");
                days::benchmark::update_readme();
            }

        }
        Command::Scaffold { day } => {info!("Scaffolding day {day}"); scaffold(day);}
    }

    //TODO Add options to scaffold files for a day

}
