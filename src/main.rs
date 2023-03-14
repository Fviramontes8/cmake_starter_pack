use clap::Parser;

// Local modules
use cmaker::{create_app_cmake, create_root_cmake};
pub mod cmaker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of CMake project
    #[arg(short, long)]
    project_name: String,

    /// Libraries to include in CMake project
    #[arg(short, long)]
    libraries: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("Project name: {}", args.project_name);

    create_app_cmake(&args.libraries);
    create_root_cmake(&args.project_name, &args.libraries);
}
