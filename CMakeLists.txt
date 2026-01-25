# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
# SPDX-FileContributor: Be <be.0@gmx.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

option(BUILD_TESTING "Build the tests for CXX-Qt-IO" ON)

project(cxx_qt_io)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(CompilerCaching)

# Enable extra Qt definitions for all projects
add_compile_definitions(
    QT_NO_CAST_FROM_ASCII
    QT_NO_CAST_TO_ASCII
    QT_NO_CAST_FROM_BYTEARRAY
    QT_NO_URL_CAST_FROM_STRING
    QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
    QT_NO_FOREACH
    QT_NO_JAVA_STYLE_ITERATORS
    QT_NO_KEYWORDS
    QT_USE_QSTRINGBUILDER
)

# QMAKE environment variable is needed by qt-build-utils to ensure that Cargo
# uses the same installation of Qt as CMake does.
find_package(Qt6 COMPONENTS Core Gui Network Test)
if(NOT Qt6_FOUND)
    message(FATAL_ERROR "cxx-qt-io only supports Qt 6 builds.")
endif()

add_definitions(-DQT_NO_CONTEXTLESS_CONNECT)

find_program(MEMORYCHECK_COMMAND valgrind)
if(NOT "${MEMORYCHECK_COMMAND}" STREQUAL "MEMORYCHECK_COMMAND-NOTFOUND")
    if (NOT WIN32)
       MESSAGE(STATUS "Valgrind found! Tests based on valgrind must be executed.")
    endif()
endif()

# Set our extra command options for valgrind
# TODO: we need to come up with a better way to suppress "possibly lost" errors.
# Suppression file doesn't work because there is a ton of mangled names that won't remain stable.
set(MEMORYCHECK_COMMAND_OPTIONS --error-exitcode=1 --errors-for-leak-kinds=definite --leak-check=full --trace-children=yes --track-origins=yes --show-possibly-lost=no)
# A suppressions file which silences errors from other libs like QtCore
set(MEMORYCHECK_SUPPRESSIONS_FILE "${CMAKE_SOURCE_DIR}/valgrind_suppressions.txt")

# Enable testing (this needs to be called before subdirs are added to detect tests in them)
enable_testing()

# Create helper method which adds a valgrind test with the given binary
function(add_valgrind_test NAME_WITH_PREFIX BINARY WORKING_DIRECTORY)
    if("${MEMORYCHECK_COMMAND}" STREQUAL "MEMORYCHECK_COMMAND-NOTFOUND")
       if (NOT WIN32)
           MESSAGE(STATUS "valgrind not found. Please install it")
       endif()
    else()
       add_test(NAME ${NAME_WITH_PREFIX}_valgrind
           COMMAND ${MEMORYCHECK_COMMAND} ${MEMORYCHECK_COMMAND_OPTIONS} --suppressions=${MEMORYCHECK_SUPPRESSIONS_FILE} --gen-suppressions=all ${BINARY}
           WORKING_DIRECTORY "${WORKING_DIRECTORY}"
       )
    endif()
endfunction()

get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
set(CARGO_ENV "QMAKE=set:${QMAKE}")
set(RUNTIME_ENV "")

# On windows, Qt dll needs to be in the PATH for the tests to run
if(CMAKE_SYSTEM_NAME STREQUAL "Windows" AND BUILD_TESTING)
    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_BINS
        OUTPUT_VARIABLE QT_INSTALL_BINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    execute_process(
        COMMAND ${QMAKE} -query QT_INSTALL_PLUGINS
        OUTPUT_VARIABLE QT_INSTALL_PLUGINS
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    list(
        APPEND
        RUNTIME_ENV
        "PATH=path_list_append:${QT_INSTALL_BINS}"
        "QT_PLUGIN_PATH=path_list_append:${QT_INSTALL_PLUGINS}"
    )
    list(APPEND CARGO_ENV ${RUNTIME_ENV})
endif()

# Same logic as in Corrosion.cmake
if(CMAKE_VS_PLATFORM_NAME)
    set(BUILD_DIR "${CMAKE_VS_PLATFORM_NAME}/$<CONFIG>")
elseif(CMAKE_CONFIGURATION_TYPES)
    set(BUILD_DIR "$<CONFIG>")
else()
    set(BUILD_DIR .)
endif()

# Set the target dir to the same that Corrosion uses to reuse build artifacts
# from the main build.
set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/${BUILD_DIR}/cargo/build")

if(BUILD_TESTING)
    add_subdirectory(tests)
    add_test(NAME cargo_tests COMMAND cargo test --release --all-features --lib --target-dir ${CARGO_TARGET_DIR})
    add_test(NAME cargo_doc COMMAND cargo doc --release --all-features --target-dir ${CARGO_TARGET_DIR})
    add_test(NAME cargo_clippy COMMAND cargo clippy --release --all-features --target-dir ${CARGO_TARGET_DIR} -- -D warnings)

    set_tests_properties(cargo_tests cargo_clippy PROPERTIES
        ENVIRONMENT_MODIFICATION "${CARGO_ENV}"
    )
    set_tests_properties(cargo_doc PROPERTIES
        ENVIRONMENT_MODIFICATION "${CARGO_ENV};RUSTDOCFLAGS=set:--deny=warnings"
    )
endif()
