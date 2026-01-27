cmake_minimum_required(VERSION 3.24)

project(tests_standalone)
set(APP_NAME ${PROJECT_NAME})

if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Core Network Test)

if(MSVC)
    set_property(
        TARGET Qt6::Core Qt6::Network Qt6::Test
        PROPERTY MAP_IMPORTED_CONFIG_DEBUG "RELEASE")
endif()

find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        GIT_TAG main
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

set(CRATE standalone)
cxx_qt_import_crate(
    MANIFEST_PATH rust/Cargo.toml
    CRATES ${CRATE}
    QT_MODULES Qt::Core Qt::Network
)

add_executable(${APP_NAME}
  cpp/main.cpp
  cpp/qbuffer.h
  cpp/qdeadlinetimer.h
  cpp/qdtlsgeneratorparameters.h
  cpp/qfile.h
)

target_include_directories(${APP_NAME} PRIVATE cpp)

target_link_libraries(${APP_NAME}
  PRIVATE
  ${CRATE}
  Qt::Test
  Qt::Core
  Qt::Network
)
