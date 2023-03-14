use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

enum Libs {
    OpenCV,
    ONNX,
    MPI,
}

impl std::str::FromStr for Libs {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opencv" => Ok(Libs::OpenCV),
            "onnx" => Ok(Libs::ONNX),
            "mpi" => Ok(Libs::MPI),
            _ => Err(format!(
                "Unknown variant: {}\nopencv, onnx, and mpi are supported",
                s
            )),
        }
    }
}

fn write_lib_to_root_cmake(content: &mut String, libs: &Vec<String>) -> () {
    for elem in libs.iter() {
        let lib: Libs = elem.as_str().to_lowercase().parse().unwrap();
        match lib {
            Libs::OpenCV => {
                let find_opencv = "\n\nfind_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})";
                content.push_str(&find_opencv);
            }
            Libs::MPI => {
                let find_mpi = "\n\nfind_package(MPI REQUIRED)
include_directories(${MPI_INCLUDE_PATH})";
                content.push_str(&find_mpi);
            }
            _ => continue,
        }
    }
}

pub fn create_root_cmake(project_name: &String, libs: &Vec<String>) -> () {
    let mut main_cmake_contents = format!(
        "cmake_minimum_required(VERSION 3.14)
project({}
    VERSION 1.0
    LANGUAGES CXX
)",
        project_name
    );
    write_lib_to_root_cmake(&mut main_cmake_contents, &libs);
    main_cmake_contents.push_str("\n\nadd_subdirectory(app)");

    let mut out_file = fs::File::create("CMakeLists.txt").unwrap();
    out_file.write_all(main_cmake_contents.as_bytes()).unwrap();

    let exe_path = env::current_dir().unwrap();
    let mut running_path = exe_path.to_str().unwrap().to_owned();
    running_path.push_str("/CMakeLists.txt");
    println!("CMakelists.txt installed in {}", running_path);
}

fn add_app_libs(content: &mut String, libs: &Vec<String>) -> () {
    for elem in libs.iter() {
        let lib: Libs = elem.as_str().to_lowercase().parse().unwrap();
        match lib {
            Libs::OpenCV => {
                let opencv_libs = "\n\tPRIVATE ${OpenCV_LIBS}";
                content.push_str(&opencv_libs);
            }
            Libs::MPI => {
                let mpi_libs = "\n\tPRIVATE ${MPI_LIBRARIES}";
                content.push_str(&mpi_libs);
            }
            Libs::ONNX => {
                let onnx_libs = "\n\tPRIVATE onnxruntime";
                content.push_str(&onnx_libs);
            }
        }
    }
}

pub fn create_app_cmake(libs: &Vec<String>) -> () {
    let app_path = Path::new("app");
    if !(app_path.exists() || app_path.is_dir()) {
        fs::create_dir(app_path).unwrap();
    }

    let mut app_cmake_contents = format!(
        "include_directories(${{PROJECT_SOURCE_DIR}})
add_executable(${{PROJECT_NAME}}
    app.cpp
)
target_link_libraries(${{PROJECT_NAME}}"
    );
    add_app_libs(&mut app_cmake_contents, &libs);
    app_cmake_contents.push_str("\n)");

    let mut out_file = fs::File::create("app/CMakeLists.txt").unwrap();
    out_file.write_all(app_cmake_contents.as_bytes()).unwrap();

    let exe_path = env::current_dir().unwrap();
    let mut running_path = exe_path.to_str().unwrap().to_owned();
    running_path.push_str("/app/CMakeLists.txt");
    println!("CMakelists.txt installed in {}", running_path);
}
