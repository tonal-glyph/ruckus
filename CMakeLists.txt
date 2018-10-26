cmake_minimum_required(VERSION 3.0.0)

# set project name
project(chuck)

set(EXECUTABLE_OUTPUT_PATH "./bin")   # artifact output dir
set(LIBRARY_OUTPUT_PATH "./lib")      # shared lib output dir

# set local includes/sources
include_directories("chuck/src/core" "chuck/src/core/lo" "chuck/src/core/regex" "chuck/src/host" "chuck/src/host/RtAudio")    # includes
file(GLOB SOURCES "chuck/src/core/*.cpp" "chuck/src/core/lo/*.c" "chuck/src/regex/*.c" "chuck/src/host/*.cpp" "chuck/src/host/RtAudio/*.cpp")  # sources
add_executable(${PROJECT_NAME} ${SOURCES})  # output artifact

## Using pkg-config -- libnotify install via `sudo apt install libnotify-dev`
#find_package(PkgConfig REQUIRED)
#pkg_search_module(LIB_NOTIFY REQUIRED libnotify)
#target_link_libraries(${PROJECT_NAME} ${LIB_NOTIFY_LIBRARIES})
#target_include_directories(${PROJECT_NAME} PUBLIC ${LIB_NOTIFY_INCLUDE_DIRS})
#target_compile_options(${PROJECT_NAME} PUBLIC ${LIB_NOTIFY_CFLAGS_OTHER})

# List all compile flags here
set(_FLAGS "-std=c++17 -O3 -march=native -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__")
set(CMAKE_C_FLAGS "${_FLAGS}")
set(CMAKE_CXX_FLAGS "${_FLAGS}")

# spit out a `compile_commands.json` file for ycm completions
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

# Add command & target to run the built artifact.
# The custom-target depends on the custom-command which
# runs the artifact with `make run`.
add_custom_command(
    OUTPUT .run.bin
    COMMAND ${PROJECT_NAME}
    COMMENT "Running ChucK build..."
)
add_custom_target(
    run
    DEPENDS .run.bin
)
add_custom_command(
    OUTPUT .leak.bin
    COMMAND valgrind --leak-check=yes --track-origins=yes ${EXECUTABLE_OUTPUT_PATH}/${PROJECT_NAME}
    COMMENT "Leak checking ChucK build..."
)
add_custom_target(
    leak
    DEPENDS .leak.bin
)

