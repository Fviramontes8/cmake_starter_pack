# CMake Starter Pack
## At a glace
Creates two files, `CMakeLists.txt` and `app/CMakeLists.txt`, they are quite barebones but it can avoid some boilerplate.

It currently supports adding the following libraries to the CMake project:

- Boost Program Options (program_options | po)
- ONNX (onnx)
- OpenCV (opencv)
- MPI (mpi)

It also support adding the following "features":

- Exporting clangd settings
- C++11/14/17/20 enforcement (a.k.a adding -std=c++xx)
- Doxygen documentation support

## Program arguments
- Name of CMake project
  - project-name/p 
- Libraries to include in CMake project
  - libraries/l 
- Features you want to include in your CMake project
  - features/f
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
Create a CMake project with the name of 'edge_detector' using Boost's program options and OpenCV
```sh
cmaker -p edge_detector -l opencv -l program_options
```

Optionally, the following works as well:
```sh
cmaker -p edge_detector -l opencv -l po
```

Creating a CMake project with the name of 'mpi_histo' using Boost's program options and mpi
```sh
cmaker -p mpi_histo -l po -l mpi
```

Create another CMake project under the name 'map_calculator' exporting clangd settings, using C++20, Boost's program options, and mpi
```sh
cmaker -p map_calculator -f clangd -f cpp20 -l po -l mpi
```
