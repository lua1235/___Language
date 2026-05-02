set(LLVM_TARGET_DEFINITIONS "${CMAKE_SOURCE_DIR}/include/dialect/UnderscoreOps.td")

# Generate Dialect Declarations
mlir_tablegen(UnderscoreDialect.h.inc -gen-dialect-decls -dialect=underscore)
mlir_tablegen(UnderscoreDialect.cpp.inc -gen-dialect-defs -dialect=underscore)

# Generate Op Declarations and Definitions
mlir_tablegen(UnderscoreOps.h.inc -gen-op-decls)
mlir_tablegen(UnderscoreOps.cpp.inc -gen-op-defs)

# Create a target for these generated files
add_public_tablegen_target(MyLangTableGen)

# Create the dialect library, and the reference to the generated td files
add_library(MyLangDialect SHARED UnderscoreDialect.cpp)
target_include_directories(MyLangDialect 
    PUBLIC 
    # handwritten header 
    "${CMAKE_SOURCE_DIR}/include"
    # tablegen'erated headers and impls
    "${PROJECT_BINARY_DIR}/lib"
)


# Ensures TableGen runs BEFORE the .cpp file is compiled.
add_dependencies(MyLangDialect MyLangTableGen)
