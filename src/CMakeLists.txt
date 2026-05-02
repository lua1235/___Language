# Gather our source files in this directory.
set(
  src_files
    "${CMAKE_CURRENT_SOURCE_DIR}/main.cpp"
)

# Build our executable from the source files.
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
add_executable(___c ${src_files})
target_include_directories(___c PUBLIC ${ANTLR_GEN_DIR})

# Ensure that the antlr4-runtime is available.
add_dependencies(___c antlr)

# Find the libraries that correspond to the LLVM components
# that we wish to use
set(LLVM_LINK_COMPONENTS Core Support)
llvm_map_components_to_libnames(llvm_libs core)
get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)

# Add the MLIR, LLVM, antlr runtime and parser as libraries to link.
target_link_libraries(___c PRIVATE
    parser
    antlr4-runtime
    ${llvm_libs}
    ${dialect_libs}
	MLIRLLVMCommonConversion
	MLIRLLVMToLLVMIRTranslation
    MLIRBuiltinToLLVMIRTranslation
)

# Symbolic link our executable to the base directory so we don't have to go searching for it.
symlink_to_bin("___c")
