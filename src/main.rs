extern crate bio;
use std::path::PathBuf;
use std::str;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
}

fn returnReverseComplement(input:&str){
    println!("Value for name: {}",input);
    let output = bio::alphabets::dna::revcomp(input.as_bytes());
    let outstr = match str::from_utf8(&output){ 
        Ok(v) => v, 
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("Reverse Complement for name: {:?}",outstr);
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        returnReverseComplement(name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }


    // Continued program logic goes here...
}