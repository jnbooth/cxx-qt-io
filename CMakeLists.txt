# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
# SPDX-FileContributor: Be <be.0@gmx.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(cxx_qt_io)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

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

find_package(Qt6 COMPONENTS Core Network Test)
if(NOT Qt6_FOUND)
    message(FATAL_ERROR "cxx-qt-io only supports Qt 6 builds.")
endif()

add_definitions(-DQT_NO_CONTEXTLESS_CONNECT)

enable_testing()

get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
set(CARGO_ENV "QMAKE=set:${QMAKE}")
set(RUNTIME_ENV "")

# On windows, Qt dll needs to be in the PATH for the tests to run
if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
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

add_subdirectory(tests)
add_test(NAME cargo_tests COMMAND cargo test --all-features --lib --target-dir ${CARGO_TARGET_DIR})
