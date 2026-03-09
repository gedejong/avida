CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

PROJECT(avida-main)

# compile with debugging symbols
if(NOT WIN32 AND NOT DEFINED ENV{AVIDA_DISABLE_BACKTRACE})
if(MSVC)
  SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /Zi")
else()
  SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g")
endif()
endif()

# Final software is built directly into the work subdirectory.
SET(EXECUTABLE_OUTPUT_PATH
  "${PROJECT_BINARY_DIR}/bin"
  CACHE PATH
  "Single output directory for building all executables."
)

SET(LIBRARY_OUTPUT_PATH
  "${PROJECT_BINARY_DIR}/lib"
  CACHE PATH
  "Built libraries are placed here before installation."
)


ADD_SUBDIRECTORY(libs/apto)
if(NOT WIN32 AND NOT DEFINED ENV{AVIDA_DISABLE_BACKTRACE})
  ADD_SUBDIRECTORY(libs/backward-cpp)
endif()
IF(NOT WIN32)
  if(NOT APPLE)
    ADD_SUBDIRECTORY(libs/tcmalloc-1.4)
  endif()
ENDIF(NOT WIN32)
ADD_SUBDIRECTORY(avida-core)

# Default location for installed software/configs/ docs is the build directory.
SET(CMAKE_INSTALL_PREFIX
  "${PROJECT_BINARY_DIR}"
  CACHE PATH
  "Install path prefix, prepended onto install directories."
  FORCE
)

