extern crate bio;
// use crate rust_htslib::bcf::{Reader,Read};
mod vcf_parser;
use std::process;
use std::str;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = "A collection of useful bioinformatics functions I've found on written up in Rust over the years")]
struct Cli {
    /// A DNA sequence to operate on
    #[clap(value_parser)]
    seq: Option<String>,

    /// Path to a vcf input file
    #[clap(short, long, value_parser, value_name = "FILE")]
    vcf: Option<String>,

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

fn return_reverse_complement(input:&str){
    println!("[STDOUT]: Value for name: {}",input);
    let output = bio::alphabets::dna::revcomp(input.as_bytes());
    let outstr = match str::from_utf8(&output){ 
        Ok(v) => v, 
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("[STDOUT]: Reverse Complement for name: {}",outstr);
}




fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(sequence) = cli.seq.as_deref() {
        return_reverse_complement(sequence);
        process::exit(0x0000);
    }

    if let Some(vcf) = cli.vcf.as_deref() {
        println!("[STDOUT]: Parsing VCF from path: {}", vcf);
        vcf_parser::parse_vcf(vcf);
    }


    // Continued program logic goes here...
}