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

    /// 'Features' to add in a CMake project
    #[arg(short, long)]
    features: Vec<String>,
}

fn main() {
    let args = Args::parse();

    create_app_cmake(&args.libraries);
    create_root_cmake(&args.project_name, &args.libraries, &args.features);
}
