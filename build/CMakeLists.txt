#
#  CMakeLists.txt
#  Copyright 2021 ItJustWorksTM
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#

cmake_minimum_required(VERSION 3.20)

project(smce-rs)

find_package(Threads REQUIRED)
find_package(SMCE 1.4 REQUIRED)

get_target_property(SMCE_INCLUDE_DIRS SMCE::SMCE INTERFACE_INCLUDE_DIRECTORIES)
get_target_property(SMCE_IMPORTED_LOCATION SMCE::SMCE LOCATION)

foreach(INCLUDE_DIR ${SMCE_INCLUDE_DIRS})
    get_filename_component(INCLUDE_DIR ${INCLUDE_DIR} DIRECTORY)
    message("header:${INCLUDE_DIR}")
endforeach()

get_filename_component(SMCE_LIB_LOOK ${SMCE_IMPORTED_LOCATION} DIRECTORY)

message("cargo:rustc-link-search=${SMCE_LIB_LOOK}")
message("cargo:rustc-link-lib=dylib=SMCE")


execute_process(
        COMMAND "${CMAKE_COMMAND}" -E tar xf "${SMCE_RESOURCES_ARK}"
        COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${SMCE_RESOURCES_ARK}" "$ENV{OUT_DIR}"
        WORKING_DIRECTORY "$ENV{OUT_DIR}"
)
