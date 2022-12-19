use std::env;

pub use clap::Parser;

pub fn get_input_path(module: &str, file: &str) -> Option<std::path::PathBuf>
{
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(error) => panic!("Problem getting executable path: {:?}", error)
    };

    exe_path.parent().
    and_then(|val| val.parent()).
    and_then(|val| val.parent()).
    map(|p| p.join(module)).
    map(|p| p.join(file))
    
}

/// Solution to Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author="Richard Muri", version, about, long_about = None)]
pub struct Args {
   /// Name of the input text file
   #[arg(short, long, default_value_t = String::from("test.txt"))]
   pub file: String
}

impl Args{
    pub fn init() -> Self
    {
        Args::parse()
    }
}

