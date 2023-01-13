# About 

I like making funny cursed things : )

I made this so I could 'use' rust for a intro to java course (obviously as a joke). and then I decided why not make it semi good and allow cursed Java RT reflection in rust. 

Now rust can run on over 3 billion devices and is "truly" system agnostic --all systems are mips now-- 


# Demo

A (bad) version of asteroids written in Rust running on the JVM

[![Demo](https://img.youtube.com/vi/1__tTtZ4K48/default.jpg)](https://www.youtube.com/watch?v=1__tTtZ4K48)

# Getting started

## Prerequisites

you need cargo installed with your native target as well as the `mips-unknown-linux-gnu` target installed
- install with 
`$ rustup target add mips-unknown-linux-gnu`

In addition you will need to have `mips-linux-gnu-ld` in your path.
- debian
`$ apt install binutils-mips-linux-gnu`

`java` and `javac` need to be in your path.

`zip` and `unzip` are also needed in your path (for now).



## Building and Running

`$ cargo build_all` will build the rust program, generate a raw bin file, build all java files, package compiled java and raw bin file into a Jar file.

the final Jar is placed under `./java_rt/out/JavaRT.jar`

The generated Jar file is standalone and does not require any library's or files in its path.

`$ cargo run_java` this will first call `build_all` and then run the generated Jar file

`$ cargo clean_all` this will use `$ cargo clean` and then clean the java build files



# Things to know

## Execution 

by deafult execution always starts at address `0x00000000` with all registers initialized to zero. `bss` sections are assumed to be initiated with all `0x00` bytes by the JVM.

Bootstrap code is present in the `rlib` library and will be placed at the starting address. 
This code initalized the values for `$sp` `$ra` `$gp` `$fp` and jumps to the `main` symbol. 
By default `main` is expected to have the signature `f() -> !` however `f()` will also work by simple exiting on return.

`$sp` is initialized to `0xFFFFFFF0` as owned memory "begins" at `0xFFFFFFFF` but the stack must be alligned to `0x8`. The stack will grow downwards until extending
pass the length of the provided memory array causing a memory exception. Because the stack starts at the highest address and grows downwards it is possible to grow
the stack or owned memory in general during runime. It is also possible to start the stack lower in memory and use the end of memory as thread local storage.

## Memory
Each Thread contains two kinds of memory `shared` and `owned`. 

### shared

`shared` memory starts at `0x00000000` to `0x7FFFFFFFF` however this is not always entierly used

`shared` memory contains all program data as well as any heap data that should be accesable across threads. 



### owned

`owned` memory starts at `0x80000000` to `0xFFFFFFFF`. The size can be specified when creating a new thread and resized during runtime.

`owned` memory is thread local it can contain any data but is setup to only be used for stack data by default. 
NOTE: program data cannot be sotred here no execution is allowed from owned memory.

By default the stack owns the entierty of `owned` memory. 

internally `owned` memory is mapped so that the vm address `0xFFFFFFFF` to memory array index `0x00000000` and `0xFFFFFFFA` to memory array index `0x00000001`. while the addresses are reversed the first 2 bits are preserved to keep the correct endian ordering. 

## Multithreading

Might add scoped threads.

Stack size is currently fixed per thread once execution has started. 

## Misc

Currently there is no "hardware" floating point support. I do plan on eventually adding it however.

# Things I might do with this

Maybe make a procedural macro that translate basic java code into rust RT reflection to make it more "ergonomic"

Actually write documentation.

Maybe try and make the NJI interface less "whoops free after use" prone

remove the need for `zip` and `unzip` to be in your path when building

Possibly try to implement unwinding

# Usage

don't (please) what possible reason besides personal hell would you want to use this.

But if you do please credit me.

## License

See LICENSE for more information.
