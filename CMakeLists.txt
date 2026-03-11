CMAKE_MINIMUM_REQUIRED(VERSION 3.20)

PROJECT(avida-main)

# Vendor apto still relies on deprecated Apple atomic APIs. Suppress this
# warning category on macOS to keep build output signal-heavy.
if(APPLE)
  add_compile_options(-Wno-deprecated-declarations)
endif()

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
if(APPLE AND TARGET aptoshared AND NOT MSVC)
  # apto's CMake can append pthread twice on some paths; normalize to one.
  set_property(TARGET aptoshared PROPERTY LINK_LIBRARIES pthread)
endif()
if(NOT WIN32 AND NOT DEFINED ENV{AVIDA_DISABLE_BACKTRACE})
  # Keep backward-cpp enabled, but avoid noisy optional library probing
  # (libdw/libbfd/libdwarf) unless explicitly requested by cache overrides.
  if(NOT DEFINED STACK_DETAILS_AUTO_DETECT)
    set(STACK_DETAILS_AUTO_DETECT FALSE CACHE BOOL "Disable backward auto detect" FORCE)
  endif()
  if(NOT DEFINED STACK_DETAILS_BACKTRACE_SYMBOL)
    set(STACK_DETAILS_BACKTRACE_SYMBOL TRUE CACHE BOOL "Use backtrace symbols in backward" FORCE)
  endif()
  if(NOT DEFINED STACK_DETAILS_DW)
    set(STACK_DETAILS_DW FALSE CACHE BOOL "Disable libdw for backward" FORCE)
  endif()
  if(NOT DEFINED STACK_DETAILS_BFD)
    set(STACK_DETAILS_BFD FALSE CACHE BOOL "Disable libbfd for backward" FORCE)
  endif()
  if(NOT DEFINED STACK_DETAILS_DWARF)
    set(STACK_DETAILS_DWARF FALSE CACHE BOOL "Disable libdwarf for backward" FORCE)
  endif()
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

