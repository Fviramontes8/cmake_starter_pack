use clap::Parser;
use std::env;
use std::fs::File;
use std::io::Write;

use libs::Libs;

pub mod libs;

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

fn write_libs_to_cmake_file(content: &mut String, libs: &Vec<String>) -> () {
    for elem in libs.iter() {
        let lib: Libs = elem.as_str().to_lowercase().parse().unwrap();
        match lib {
            Libs::OpenCV => {
                let find_opencv = "\nfind_package(OpenCV REQUIRED)";
                content.push_str(&find_opencv);
            }
            Libs::MPI => {
                let find_mpi = "\nfind_package(MPI REQUIRED)";
                content.push_str(&find_mpi);
            }
            _ => continue,
        }
        println!("{} ", elem);
    }
}

fn create_root_cmake_file(project_name: &String, libs: &Vec<String>) -> () {
    let mut out_file = File::create("CMakeLists.txt").unwrap();
    let mut main_cmake_contents = format!(
        "cmake_minimum_required(VERSION 3.14)
project({}
    VERSION 1.0
    LANGUAGES CXX
)",
        project_name
    );
    write_libs_to_cmake_file(&mut main_cmake_contents, &libs);
    out_file.write_all(main_cmake_contents.as_bytes()).unwrap();

    let exe_path = env::current_dir().unwrap();
    let mut running_path = exe_path.to_str().unwrap().to_owned();
    running_path.push_str("/CMakeLists.txt");
    println!("CMakelists.txt installed in {}", running_path);
}

fn main() {
    let args = Args::parse();

    println!("Project name: {}", args.project_name);
    create_root_cmake_file(&args.project_name, &args.libraries);

    for elem in args.libraries.iter() {
        println!("{} ", elem);
    }
}
