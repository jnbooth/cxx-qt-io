# SPDX-FileCopyrightText: 2021 Tenacity Audio Editor contributors
# SPDX-FileContributor: Be <be.0@gmx.com>
# SPDX-FileContributor: Emily Mabrey <emabrey@tenacityaudio.org>
# SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: BSD-3-Clause
#[=======================================================================[.rst:
CompilerCaching
---------------

Search for sccache and ccache and use them for compiler caching for C & C++.
sccache is preferred if both are found, but the user can override this by
explicitly setting SCCACHE=OFF to use ccache when both are installed.
#]=======================================================================]

# ccache does not support MSVC
if(NOT CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
  find_program(CCACHE_PROGRAM ccache)
  mark_as_advanced(FORCE CCACHE_PROGRAM)
  if("${CCACHE_PROGRAM}" STREQUAL "CCACHE_PROGRAM-NOTFOUND")
    message(STATUS "Could NOT find ccache")
  else()
    message(STATUS "Found ccache: ${CCACHE_PROGRAM}")
    option(CCACHE "Use ccache for compiler caching to speed up rebuilds." ON)
  endif()
endif()

find_program(SCCACHE_PROGRAM sccache)
mark_as_advanced(FORCE SCCACHE_PROGRAM)
if("${SCCACHE_PROGRAM}" STREQUAL "SCCACHE_PROGRAM-NOTFOUND")
  message(STATUS "Could NOT find sccache")
else()
  message(STATUS "Found sccache: ${SCCACHE_PROGRAM}")
  option(SCCACHE "Use sccache for compiler caching to speed up rebuilds." ON)
endif()

if(SCCACHE)
  message(STATUS "Using sccache for compiler caching to speed up rebuilds")
  set(CMAKE_C_COMPILER_LAUNCHER "${SCCACHE_PROGRAM}")
  set(CMAKE_CXX_COMPILER_LAUNCHER "${SCCACHE_PROGRAM}")
  if (NOT DEFINED ENV{RUSTC_WRAPPER})
    # Enable sccache for rustc - especially important when building cxx-qt-lib!
    set(CMAKE_RUSTC_WRAPPER "${SCCACHE_PROGRAM}" CACHE PATH "RUSTC_WRAPPER detected by CMake")
  endif()

  # Instruct MSVC to generate symbolic debug information within object files for sccache
  if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    if(IS_MULTI_CONFIG)
      foreach(CONFIG ${CMAKE_CONFIGURATION_TYPES})
        string(TOUPPER "${CONFIG}" CONFIG)
        string(REPLACE "/Zi" "/Z7" CMAKE_CXX_FLAGS_${CONFIG} "${CMAKE_CXX_FLAGS_${CONFIG}}")
        string(REPLACE "/Zi" "/Z7" CMAKE_C_FLAGS_${CONFIG} "${CMAKE_C_FLAGS_${CONFIG}}")
      endforeach()
    else()
      string(TOUPPER "${CMAKE_BUILD_TYPE}" CONFIG)
      string(REPLACE "/Zi" "/Z7" CMAKE_CXX_FLAGS_${CONFIG} "${CMAKE_CXX_FLAGS_${CONFIG}}")
      string(REPLACE "/Zi" "/Z7" CMAKE_C_FLAGS_${CONFIG} "${CMAKE_C_FLAGS_${CONFIG}}")
    endif()
  endif()
elseif(CCACHE)
  message(STATUS "Using ccache for compiler caching to speed up rebuilds")
  set(CMAKE_C_COMPILER_LAUNCHER "${CCACHE_PROGRAM}")
  set(CMAKE_CXX_COMPILER_LAUNCHER "${CCACHE_PROGRAM}")
else()
  if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    message(STATUS "No compiler caching enabled. Install sccache to speed up rebuilds.")
  else()
    message(STATUS "No compiler caching enabled. Install ccache or sccache to speed up rebuilds.")
  endif()
endif()
