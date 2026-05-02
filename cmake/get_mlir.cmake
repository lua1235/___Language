# Need MLIR so we can link against it. Either we automatically find this in the "default" place or
# it's found because we set up $MLIR_DIR. There should be no additions necessary here.
find_package(LLVM REQUIRED CONFIG)
find_package(MLIR REQUIRED CONFIG)

# Status messages about LLVM found.
message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using Config.cmake in: ${MLIR_DIR}")

# Add mlir specific pieces to our build.
include_directories("${MLIR_INCLUDE_DIRS}")
include_directories("${LLVM_INCLUDE_DIRS}")
add_definitions("${MLIR_DEFINITIONS}")

# Import the TableGen Macros
# This allows you to use mlir_tablegen() just like the internal build
list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
include(AddLLVM)
include(AddMLIR)
include(TableGen)
