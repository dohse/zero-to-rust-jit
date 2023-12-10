# From Zero to Rust JIT with the LLVM C-API

LLVM C-API demo for runtime compilation with OrcJIT: IRBuilder ➜ C ➜ Rust

## Build against LLVM release version

Install LLVM 17 with `apt` and build:
```
➜ wget https://apt.llvm.org/llvm.sh
➜ chmod +x llvm.sh
➜ sudo ./llvm.sh 17
➜ sudo apt install -y libzstd-dev
➜ git clone https://github.com/weliveindetail/llvm-jit-capi
➜ rustc --version -v | tail -1
LLVM version: 17.0.4
➜ cd llvm-jit-capi
➜ CC=clang CXX=clang++ ./build.sh build-release17 /usr/lib/llvm-17/lib/cmake/llvm
```

## Build against LLVM mainline

Build mainline LLVM from source. This will take a while. We are using the C-API so we can easily switch to `Release` mode if we don't need to debug into LLVM (doesn't affect debugging of JITed code):
```
➜ git clone https://github.com/llvm/llvm-project
➜ cmake -GNinja -Sllvm-project/llvm -Bllvm-project/build \
        -DCMAKE_BUILD_TYPE=RelWithDebInfo \
        -DLLVM_TARGETS_TO_BUILD=host \
        -DLLVM_USE_LINKER=lld
➜ cd llvm-project/build
➜ ninja llvm-config
➜ ninja $(bin/llvm-config --libnames OrcJIT native)
```

Build the project against the just-built LLVM with the rustc version check disabled:
```
➜ git clone https://github.com/weliveindetail/llvm-jit-capi
➜ cd llvm-jit-capi
➜ rustc --version -v | tail -1
LLVM version: 17.0.4
➜ CC=clang CXX=clang++ cmake -GNinja -Bbuild-mainline -S. \
    -DCMAKE_BUILD_TYPE=Debug \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=On \
    -DLLVM_DIR=/path/to/llvm-project/build/lib/cmake/llvm \
    -DRUSTC_VERSION_CHECK=Off
➜ ninja -Cbuild-mainline
```

Debug the JITed bitcode:
```
➜ lldb -- build-mainline/llvm-jit-c
(lldb) version
lldb version 15.0.7
(lldb) b sum.rs:4
Breakpoint 1: no locations (pending).
(lldb) run build-mainline/sum_rs.bc
Process 235912 launched (x86_64)
Redirecting to host function RustPanicHandler @ 0x0000555555e89180: _ZN4core9panicking5panic17hcad0f3a89a1b36aaE
1 location added to breakpoint 1
Process 235912 stopped
* thread #1, name = 'llvm-jit-c', stop reason = breakpoint 1.1
    frame #0: 0x00007ffff7fba008 JIT(0x7ffff7ffa000)`sum(a=-2147483648, b=-2147483648) at sum.rs:4:5
   1    #[no_mangle]
   2    pub fn sum(a: i32, b: i32) -> i32 {
   3        //println!("Hello Rust!");
-> 4        a + b
   5    }
(lldb) c
Process 235912 resuming
Panic due to overflow: attempt to add with overflow
Process 235912 exited with status = 1
(lldb)
```

Run Rust

```
cargo run build/sum_c.bc
```
