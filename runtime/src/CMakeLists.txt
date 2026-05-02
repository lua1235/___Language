# Gather our source files in this directory.
set(
  gazprea_rt_files
        runtime.cpp
)

# Build our executable from the source files.
add_library(___rt SHARED ${gazprea_rt_files})
target_include_directories(___rt PUBLIC ${RUNTIME_INCLUDE})

# Symbolic link our library to the base directory so we don't have to go searching for it.
symlink_to_bin("___rt")
