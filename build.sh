#!/usr/bin/env bash
cd build
cmake -G Ninja ../CMakeLists.txt
cd ..
ninja