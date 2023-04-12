# CMake Starter Pack
## At a glace
Creates two files, `CMakeLists.txt` and `app/CMakeLists.txt`, they are quite barebones but it can avoid some boilerplate.

It currently supports adding the following libraries to the CMake project:

- Boost Program Options (program_options | po)
- ONNX (onnx)
- OpenCV (opencv)
- MPI (mpi)

## Program arguments
- Name of CMake project
  - project-name/p 
- Libraries to include in CMake project
  - libraries/l 
- Print help
  - help/h 
- Print version
  - version/V 

## Build instructions
```sh
cargo build
```

## Install instuctions
```sh
cargo install --path .
```

## Usage examples
Create a cmake project with the name of 'edge_detector' using Boost's program options and OpenCV
```sh
cmaker -p edge_detector -l opencv -l program_options
```

Optionally, the following works as well:
```sh
cmaker -p edge_detector -l opencv -l po
```

Creating a cmake project with the name of 'mpi_histo' using Boost's program options and mpi
```sh
cmaker -p mpi_histo -l po -l mpi
```
