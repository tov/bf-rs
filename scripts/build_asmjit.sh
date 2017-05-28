#/bin/sh -e

git clone https://github.com/asmjit/asmjit
cd asmjit

mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release -DASMJIT_STATIC=1 ..
make
