# About 

I like making funny cursed things : )

I made this so I could 'use' rust for a intro to java course (obviously as a joke). and then I decided why not make it semi good and allow cursed Java RT reflection in rust. 

Now rust can run on over 3 billion devices and is "truly" system agnostic --all systems are mips now-- 

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

`$sp` is initialized by first preforming a system call to get the length of `owned` memory then adding `0x80000000`. the stack grows downward towards `0x80000000`.
It is reccomened that `shared` memory be kept a few bytes shorter than its maximum capacity as the buffer between the end of `shared` memory and the start of `owned` memory act as stack overflow protection. When the stack overflows java will throw a runtime exception of index out of bounds and execution of that thread will immediatly be ended.

It would be nice if the stack uh grew upwards so you could dynamically increase the stack size but we can't all have what we want

## Memory
Each Thread contains two kinds of memory `shared` and `owned`. 

### shared

`shared` memory starts at `0x00000000` to `0x7FFFFFFFF` however this is not always entierly used

`shared` memory contains all program data as well as any heap data that should be accesable across threads. 



### owned

`owned` memory starts at `0x80000000` to `0xFFFFFFFF`. The size can be specified when creating a new thread

`owned` memory is thread local it can contain any data but is setup to only be used for stack data by default. 
NOTE: program data cannot be sotred here no execution is allowed from owned memory.

By default the stack owns the entierty of `owned` memory. 

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
