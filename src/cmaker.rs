use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use features::Features;
mod features;
use libraries::Libraries;
mod libraries;

fn write_lib_to_root_cmake(content: &mut String, libs: &[String]) {
    for elem in libs.iter() {
        let lib: Libraries = elem.as_str().to_lowercase().parse().unwrap();
        match lib {
            Libraries::BoostProgramOptions => {
                let addendum = "\n\nfind_package(Boost REQUIRED COMPONENTS program_options)
include_directories(${Boost_INCLUDE_DIR})";
                content.push_str(addendum);
            }
            Libraries::OpenCV => {
                let addendum = "\n\nfind_package(OpenCV REQUIRED)
include_directories(${OpenCV_INCLUDE_DIRS})";
                content.push_str(addendum);
            }
            Libraries::Mpi => {
                let addendum = "\n\nfind_package(MPI REQUIRED)
include_directories(${MPI_INCLUDE_PATH})";
                content.push_str(addendum);
            }
            _ => continue,
        }
    }
}

fn write_feats_to_root_cmake(content: &mut String, feats: &[String]) {
    for elem in feats.iter() {
        let feat: Features = elem.as_str().to_lowercase().parse().unwrap();
        match feat {
            Features::Clangd => {
                let addendum = "\n\nset(CMAKE_EXPORT_COMPILE_COMMANDS ON)";
                content.push_str(addendum);
            }
            Features::Cpp11 => {
                let addendum = "\n\nset(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)";
                content.push_str(addendum);
            }
            Features::Cpp14 => {
                let addendum = "\n\nset(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)";
                content.push_str(addendum);
            }
            Features::Cpp17 => {
                let addendum = "\n\nset(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)";
                content.push_str(addendum);
            }
            Features::Cpp20 => {
                let addendum = "\n\nset(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)";
                content.push_str(addendum);
            }
            Features::Doxygen => {
                let addendum = "\n\n
# Taken from https://vicrucann.github.io/tutorials/quick-cmake-doxygen/
find_package(Doxygen)
if (DOXYGEN_FOUND)
  set(DOXYGEN_IN ${CMAKE_CURRENT_SOURCE_DIR}/docs/Doxyfile.in)
  set(DOXYGEN_OUT ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile)

  configure_file(${DOXYGEN_IN} ${DOXYGEN_OUT} @ONLY)
  message(\"Doxygen build started\")

  add_custom_target(doc_doxygen ALL
    COMMAND ${DOXYGEN_EXECUTABLE} ${DOXYGEN_OUT}
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT \"Generating project documentation with Doxygen\"
    VERBATIM
)
else (DOXYGEN_FOUND)
  message(\"Doxygen needs to be installed to generate documention\")
endif (DOXYGEN_FOUND)
";
                content.push_str(addendum);
            }
        }
    }
}

pub fn create_root_cmake(project_name: &String, libs: &[String], feats: &[String]) {
    let mut main_cmake_contents = format!(
        "cmake_minimum_required(VERSION 3.14)
project({}
    VERSION 0.1.0
    LANGUAGES CXX
)",
        project_name
    );
    write_feats_to_root_cmake(&mut main_cmake_contents, feats);
    write_lib_to_root_cmake(&mut main_cmake_contents, libs);
    main_cmake_contents.push_str("\n\nadd_subdirectory(app)");

    let mut out_file = fs::File::create("CMakeLists.txt").unwrap();
    out_file.write_all(main_cmake_contents.as_bytes()).unwrap();

    let exe_path = env::current_dir().unwrap();
    let mut running_path = exe_path.to_str().unwrap().to_owned();
    running_path.push_str("/CMakeLists.txt");
    println!("CMakelists.txt installed in {}", running_path);
}

fn add_app_libs(content: &mut String, libs: &[String]) {
    for elem in libs.iter() {
        let lib: Libraries = elem.as_str().to_lowercase().parse().unwrap();
        match lib {
            Libraries::BoostProgramOptions => {
                let po_libs = "\n\tPRIVATE ${Boost_LIBRARIES}";
                content.push_str(po_libs);
            }
            Libraries::OpenCV => {
                let opencv_libs = "\n\tPRIVATE ${OpenCV_LIBS}";
                content.push_str(opencv_libs);
            }
            Libraries::Mpi => {
                let mpi_libs = "\n\tPRIVATE ${MPI_LIBRARIES}";
                content.push_str(mpi_libs);
            }
            Libraries::Onnx => {
                let onnx_libs = "\n\tPRIVATE onnxruntime";
                content.push_str(onnx_libs);
            }
        }
    }
}

pub fn create_app_cmake(libs: &[String]) {
    let app_path = Path::new("app");
    if !(app_path.exists() || app_path.is_dir()) {
        fs::create_dir(app_path).unwrap();
    }

    let mut app_cmake_contents = String::from(
        "include_directories(${PROJECT_SOURCE_DIR})
add_executable(${PROJECT_NAME}
    main.cpp
)

target_link_libraries(${PROJECT_NAME}",
    );
    add_app_libs(&mut app_cmake_contents, libs);
    app_cmake_contents.push_str("\n)");

    let mut out_file = fs::File::create("app/CMakeLists.txt").unwrap();
    out_file.write_all(app_cmake_contents.as_bytes()).unwrap();

    let exe_path = env::current_dir().unwrap();
    let mut running_path = exe_path.to_str().unwrap().to_owned();
    running_path.push_str("/app/CMakeLists.txt");
    println!("CMakelists.txt installed in {}", running_path);
}

pub fn create_clang_format() {
    let mut out_file = fs::File::create(".clang-format").unwrap();
    let clang_format_contents = String::from(
        "BasedOnStyle: LLVM
IndentWidth: 4",
    );
    out_file
        .write_all(clang_format_contents.as_bytes())
        .unwrap();
}
