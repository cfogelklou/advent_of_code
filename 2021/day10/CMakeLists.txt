cmake_minimum_required(VERSION 3.5)
project(advent_2021_day_10)
enable_testing()

set(CMAKE_CXX_STANDARD 14)

set(ROOT_SRC ${CMAKE_CURRENT_SOURCE_DIR}/../..)
set(EXTERN_LIBS ${ROOT_SRC}/extern)

file(GLOB GOOGLETEST_SRC
  ${ROOT_SRC}/extern/googletest/googletest/src/gtest-all.cc
  ${ROOT_SRC}/extern/googletest/googlemock/src/gmock-all.cc
)

add_definitions(-DSODIUM_STATIC -DDEBUG -D_CONSOLE -DNATIVE_LITTLE_ENDIAN)

if (WIN32)
elseif (APPLE)
    set(CMAKE_BUILD_TYPE Debug)
elseif (UNIX)
    set(CMAKE_BUILD_TYPE Debug)
endif ()

file(GLOB LOCAL_SRC
    ${CMAKE_CURRENT_SOURCE_DIR}/*.c
    ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/*.h
    ${CMAKE_CURRENT_SOURCE_DIR}/*.hpp
	${CMAKE_CURRENT_SOURCE_DIR}/CMakeLists.txt
)


set(SOURCE_FILES
	# Google Test / Mock
	${GOOGLETEST_SRC}

	${LOCAL_SRC}

)

list(REMOVE_DUPLICATES SOURCE_FILES)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${ROOT_SRC}
    ${EXTERN_LIBS}/googletest/googletest
    ${EXTERN_LIBS}/googletest/googletest/include
    ${EXTERN_LIBS}/googletest/googlemock
    ${EXTERN_LIBS}/googletest/googlemock/include
)

add_executable(${PROJECT_NAME} ${SOURCE_FILES})

find_package(Threads REQUIRED)

# target_link_libraries(${PROJECT_NAME} curl)
if (WIN32)
    target_link_libraries(${PROJECT_NAME} ws2_32)
else (WIN32)
    target_link_libraries(${PROJECT_NAME} Threads::Threads)
endif (WIN32)

enable_testing()
add_test(${PROJECT_NAME} ${PROJECT_NAME})
