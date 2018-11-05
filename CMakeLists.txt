cmake_minimum_required(VERSION 3.0.0)

# set project name
project(ChucK)

# List all compile flags here
set(_CFLAGS "-O3 -march=native -std=c11 -export-dynamic -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -lstdc")
set(_CXXFLAGS "-O3 -march=native -std=c++17 -export-dynamic -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__PLATFORM_LINUX__ -O3 -fno-strict-aliasing -D__CK_SNDFILE_NATIVE__ -lstdc++ -lpthread -lm")
set(CMAKE_C_FLAGS "${_CFLAGS}")
set(CMAKE_CXX_FLAGS "${_CXXFLAGS}")
set(ENV{CC} "${clang}")
set(ENV{CXX} "${clang}")
set(ENV{LD} "${clang}")

# set local includes/sources
include_directories("chuck/src/core" "chuck/src/core/lo" "chuck/src/core/regex" "chuck/src/host" "chuck/src/host/RtAudio")    # includes
file(GLOB SOURCES "chuck/src/core/*.c" "chuck/src/core/*.cpp" "chuck/src/core/*.h" "chuck/src/core/lo/*.c" "chuck/src/core/lo/*.h" "chuck/src/core/regex/*.c" "chuck/src/core/regex/*.h" "chuck/src/host/*.cpp" "chuck/src/host/*.h" "chuck/src/host/RtAudio/*.cpp" "chuck/src/host/RtAudio/*.h")  # sources
set(SRCS "chuck/src/host/*.cpp" "chuck/src/host/RtAudio/*.cpp" "chuck/src/core/*.c" "chuck/src/core/*.cpp" "chuck/src/core/lo/*.c" "chuck/src/core/regex/*.c")
add_executable("chuck-bin" "chuck/src/core/chuck.cpp" "chuck/src/core/chuck_absyn.cpp" "chuck/src/core/chuck_carrier.cpp" "chuck/src/core/chuck_dl.cpp" "chuck/src/core/chuck_emit.cpp" "chuck/src/core/chuck_errmsg.cpp" "chuck/src/core/chuck_frame.cpp" "chuck/src/core/chuck_instr.cpp" "chuck/src/core/chuck_io.cpp" "chuck/src/core/chuck_lang.cpp" "chuck/src/core/chuck_oo.cpp" "chuck/src/core/chuck_otf.cpp" "chuck/src/core/chuck_parse.cpp" "chuck/src/core/chuck_scan.cpp" "chuck/src/core/chuck_shell.cpp" "chuck/src/core/chuck_stats.cpp" "chuck/src/core/chuck_symbol.cpp" "chuck/src/core/chuck_table.cpp" "chuck/src/core/chuck_type.cpp" "chuck/src/core/chuck_ugen.cpp" "chuck/src/core/chuck_utils.cpp" "chuck/src/core/chuck_vm.cpp" "chuck/src/core/hidio_sdl.cpp" "chuck/src/core/midiio_rtmidi.cpp" "chuck/src/core/rtmidi.cpp" "chuck/src/core/uana_extract.cpp" "chuck/src/core/uana_xform.cpp" "chuck/src/core/ugen_filter.cpp" "chuck/src/core/ugen_osc.cpp" "chuck/src/core/ugen_stk.cpp" "chuck/src/core/ugen_xxx.cpp" "chuck/src/core/ulib_machine.cpp" "chuck/src/core/ulib_math.cpp" "chuck/src/core/ulib_opsc.cpp" "chuck/src/core/ulib_regex.cpp" "chuck/src/core/ulib_std.cpp" "chuck/src/core/util_buffers.cpp" "chuck/src/core/util_console.cpp" "chuck/src/core/util_hid.cpp" "chuck/src/core/util_opsc.cpp" "chuck/src/core/util_serial.cpp" "chuck/src/core/util_string.cpp" "chuck/src/core/util_thread.cpp" "chuck/src/host/chuck_audio.cpp" "chuck/src/host/chuck_console.cpp" "chuck/src/host/chuck_main.cpp")  # output artifact

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

