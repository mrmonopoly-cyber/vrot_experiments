git submodule update --init --recursive

ROOT=$(pwd)

#wasi-sdk

cd ./ThirdParty
curl -fL -O https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-34/wasi-sdk-34.0-x86_64-linux.tar.gz
tar -xf wasi-sdk-34.0-x86_64-linux.tar.gz 
mv wasi-sdk-34.0-x86_64-linux wasi-sdk
cd ${ROOT}

#wamrc
git -C ./ThirdParty/wamr/wasm-micro-runtime/ apply ../wamrc_build.patch
cd ./ThirdParty/wamr/wasm-micro-runtime/wamr-compiler/

./build_llvm.sh
cmake -S . -B build
cmake --build build

cd ${ROOT}
git -C ./ThirdParty/wamr/wasm-micro-runtime/ apply -R ../wamrc_build.patch


