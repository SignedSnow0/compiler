# Compiler

## Building
The create `inkwell` requires some dependencies to build correctly:
* Install `llvm-18` libs using
``` bash
wget https://apt.llvm.org/llvm.sh \
  && chmod u+x llvm.sh \
  && sudo ./llvm.sh 18 \
  && sudo apt install libpolly-18-dev libz-dev
```
* Install `libzstd-dev`
``` bash
sudo apt install zstd libzstd-dev
```
* Build the program running `cargo build`