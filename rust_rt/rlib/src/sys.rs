use core::arch::asm;

//Basic mips stuff

/// Does not take reguments nor return anything
///
/// Halts the VM
pub const HALT: u32 = 0;

/// Print a 2's complement i32 to standard output
///
/// Register 4: i32 value
pub const PRINT_DEC_NUMBER: u32 = 1;

/// Get number of instructions ran
///
/// Register 2: upper half of instruction count
/// Register 3: lower half of instruction count
pub const GET_INSTRUCTIONS_RAN: u32 = 3;

/// Print a C-String ending in a \0 byte.
///
/// Register 4: ptr to begining of string
pub const PRINT_C_STRING: u32 = 4;

/// Print a char to standard output
///
/// Register 4: the char to print
pub const PRINT_CHAR: u32 = 5;

/// Sleep for x ms
///
/// Register 4: the number of ms to sleep for
pub const SLEEP_MS: u32 = 50;

/// Sleep for delta x ms
///
/// Register 4: the number of ms to sleep for munis the time it took since the last call
pub const SLEEP_D_MS: u32 = 51;

/// Current time nanos
///
/// Register 2: upper half of nanos
/// Register 3: lower half of nanos
pub const CURRENT_TIME_NANOS: u32 = 60;

/// Generate a random number between xi32 and yi32
///
/// Register 4: xi32 lower bound
/// Register 4: yi32 upper bound
///
/// Register 2: generated random number
pub const GENERATE_THREAD_RANDOM_NUMBER: u32 = 99;

//JVM interface

/// "Frees" the JVM object of the given id
///
/// Register 4: JVM Object id
pub const FREE_JVM_OBJECT: u32 = 100;

/// Get the class of a JVM Object
///
/// Register 4: JVM Object id
///
/// Register 2: JVM Object id of class
pub const GET_OBJECT_CLASS: u32 = 101;

/// Construct a JVM String
///
/// Register 4: ptr to start of naitive str
/// Register 5: len of string
///
/// Register 2: JVM id of String
pub const CREATE_JVM_STRING: u32 = 102;

/// Get the full name of the class
///
/// Register 4: JVM Object id
///
/// Register 2: JVM id of String
/// Register 3: Length of String in Register 2
pub const GET_FULL_CLASS_NAME: u32 = 103;

/// Copy a JVM Strings content into memory
///
/// Register 4: JVM Object id. Must be a valid ID pointing to a valid JVM String Object
/// Register 5: Pointer to start of buffer
/// Register 6: Maximum number of bytes to be copied into buffer
///
/// Register 2: Number of bytes copied into buffer
pub const COPY_FROM_JVM_STRING: u32 = 104;

/// Get the String representation of the JVM Object
///
/// Register 4: JVM Object id.
///
/// Register 2: JVM Object id to string.
/// Register 3: Length of String
pub const JVM_OBJECT_TO_STRING: u32 = 105;

/// Get the length of the String
///
/// Register 4: JVM String id
///
/// Register 2: Length of JVM String
pub const JVM_STRING_LENGTH: u32 = 106;

/// Get the length of an array of objects
///
/// Register 4: JVM Object id
///
/// Register 2: Number of elements in array.
pub const JVM_ARRAY_LENGTH: u32 = 107;

/// Move JVM Object array into a "Naitive" array
///
/// Register 4: JVM Object array id
/// Register 5: ptr to start of array
/// Register 6: max number of ELEMENTS in array ie 1 means there are 4 bytes of avaliable space
///
/// Register 2: Number of elements copied over
pub const MOVE_INTO_NAITIVE_ARRAY: u32 = 108;

/// Get an array of declaired fields defined in the class
///
/// Register 4: JVM Object id (must be a valid Class)
///
/// Register 2: JVM Object id of fields array
/// Register 3: Length of fields array.
pub const GET_DECLAIRED_FIELDS_OF_JVM_CLASS: u32 = 109;

/// Get an array of declaired methods defined in the class
///
/// Register 4: JVM Object id (must be a valid Class)
///
/// Register 2: JVM Object id of methods array
/// Register 3: Length of methods array.
pub const GET_DECLAIRED_METHODS_OF_JVM_CLASS: u32 = 110;

/// Get an array of declaired constructors defined in the class
///
/// Register 4: JVM Object id (must be a valid Class)
///
/// Register 2: JVM Object id of constructor array
/// Register 3: Length of constructor array.
pub const GET_DECLAIRED_CONSTRUCTORS_OF_JVM_CLASS: u32 = 111;

/// Create a new object array
///
/// Register 4: Length of array to create
///
/// Register 2 JVM Object id of array
pub const CREATE_NEW_OBJECT_ARRAY: u32 = 112;

/// Take the Object in the Object array and replace it with null
///
/// Register 4: JVM Object id of array
/// Register 5: Index into array
///
/// Register 2: JVM Object id of item in array
pub const TAKE_OBJECT_AT_INDEX: u32 = 113;

/// Put Object into Object array
///
/// Register 4: JVM Object id of array
/// Register 5: Index into array
/// Register 6: JVM Object to put into array
pub const PUT_OBJECT_AT_INDEX: u32 = 114;

/// Get class from name
///
/// Register 4: Ptr to start of str
/// Register 5: length of str
///
/// Register 2: A JVM Object id of a class object, or zero if the class doesnt exist
pub const CLASS_FOR_NAME: u32 = 115;

/// Get a declaired method in a class
///
/// Register 4: Class JVM Object id
/// Register 5: String JVM Object id for method name
/// Register 6: Class[] JVM Object id for parameters
///
/// Register 2: Method JVM Object id
pub const GET_CLASS_METHOD: u32 = 116;

/// Get a declaired constructor in a class
///
/// Register 4: Class JVM Object id
/// Register 5: Class[] JVM Object id for parameters
///
/// Register 2: Constructor JVM Object id
pub const GET_CLASS_CONSTRUCTOR: u32 = 117;

/// Get a declaired field in a class
///
/// Register 4: Class JVM OBject id
/// Register 5: String JVM Object id for field name
///
/// Register 2: Field JVM Object id
pub const GET_CLASS_FIELD: u32 = 118;

/// Invoke a method
///
/// Register 4: Method JVM Object id
/// Register 5: Null or Object to invoke method on
/// Register 6: Object[] JVM id for arguments
///
/// Register 2: return result Null or a valid object
/// Register 3: If non zero an error occured and this is the JVM Object id of the error
pub const INVOKE_METHOD: u32 = 119;

pub const CREATE_JVM_BOOLEAN: u32 = 150;
pub const CREATE_JVM_BYTE: u32 = 151;
pub const CREATE_JVM_SHORT: u32 = 152;
pub const CREATE_JVM_CHAR: u32 = 153;
pub const CREATE_JVM_INT: u32 = 154;
pub const CREATE_JVM_LONG: u32 = 155;
pub const CREATE_JVM_FLOAT: u32 = 156;
pub const CREATE_JVM_DOUBLE: u32 = 157;

pub const GET_JVM_BOOLEAN: u32 = 158;
pub const GET_JVM_BYTE: u32 = 159;
pub const GET_JVM_SHORT: u32 = 160;
pub const GET_JVM_CHAR: u32 = 161;
pub const GET_JVM_INT: u32 = 162;
pub const GET_JVM_LONG: u32 = 163;
pub const GET_JVM_FLOAT: u32 = 164;
pub const GET_JVM_DOUBLE: u32 = 165;

pub const GET_JVM_BOOLEAN_CLASS: u32 = 166;
pub const GET_JVM_BYTE_CLASS: u32 = 167;
pub const GET_JVM_SHORT_CLASS: u32 = 168;
pub const GET_JVM_CHAR_CLASS: u32 = 168;
pub const GET_JVM_INT_CLASS: u32 = 170;
pub const GET_JVM_LONG_CLASS: u32 = 171;
pub const GET_JVM_FLOAT_CLASS: u32 = 172;
pub const GET_JVM_DOUBLE_CLASS: u32 = 173;

/// Create a new JVM objest
///
/// Register 2: JVM Object id of Turtle
pub const CREATE_TURTLE: u32 = 200;

/// Set speed of Turtle
///
/// Register 4: JVM Object id of Turtle
/// Register 5: Speed represented as i32
pub const SET_TURTLE_SPEED: u32 = 201;

/// Calls Turtle.penDown()
///
/// Register 4: JVM Object id of Turtle
pub const TURTLE_PEN_DOWN: u32 = 202;

/// Calls Turtle.penUp()
///
/// Register 4: JVM Object id of Turtle
pub const TURTLE_PEN_UP: u32 = 203;

/// Moves the Turtle forward by x ammount
/// x is a f64
///
/// Register 4: JVM Object id of Turtle
/// Register 5: lower 32 bits of x
/// Register 6: upper 32 bits of x
pub const TURTLE_FORWARD: u32 = 204;

/// Moves the Turtle left by x ammount
/// x is a f64
///
/// Register 4: JVM Object id of Turtle
/// Register 5: lower 32 bits of x
/// Register 6: upper 32 bits of x
pub const TURTLE_LEFT: u32 = 205;

/// Moves the Turtle right by x ammount
/// x is a f64
///
/// Register 4: JVM Object id of Turtle
/// Register 5: lower 32 bits of x
/// Register 6: upper 32 bits of x
pub const TURTLE_RIGHT: u32 = 206;

/// Create a new turtle display with an associated turtle
///
/// Register 4: JVM Object id of Turtle
///
/// Register 2: JVM Object id of TurtleDisplayer
pub const CREATE_TURTLE_DISPLAY_WITH_TURTLE: u32 = 300;

/// Close TurtleDisplayer
///
/// Register 4: JVM Object id of TurtleDisplayer
pub const CLOSE_TURTLE_DISPLAYER: u32 = 301;

/// sends a draw call. the command will be copyed to the 'gpu' in our case a different thread
/// and executed there. the VM will be blocked if another draw call is requested before the previouse one has completed
/// now you may ask what the format of the draw call data is in... and to that i say ,,,, idk read screen.rs :)
///
/// Register 4: ptr to begining of draw call data
/// Register 5: length of draw call data
pub const SEND_MAIN_SCREEN_DRAW_CALL: u32 = 400;

/// is a key (represented by a char) pressed
///
/// Register 4: char to query
///
/// Register 2: 1 if its presssed 0 otherwise
pub const IS_KEY_PRESSED: u32 = 401;

/// Get width and height of screen
///
/// Register 2: width of the screen in pixels
/// Register 3: height of the screen in pixels
pub const SCREEN_WIDTH_HEIGHT: u32 = 402;

#[inline(always)]
pub fn halt() -> ! {
    unsafe {
        syscall_v_v::<HALT>();
    }

    unsafe {
        core::hint::unreachable_unchecked();
    }
}

#[inline(always)]
pub fn print_i32(num: i32) {
    unsafe {
        syscall_s_v::<PRINT_DEC_NUMBER>(num as u32);
    }
}

#[inline(always)]
pub fn get_instructions_ran() -> u64 {
    unsafe { syscall_v_d::<GET_INSTRUCTIONS_RAN>() }
}

#[inline(always)]
pub fn print_zero_term_str(str: &str) {
    unsafe {
        syscall_s_v::<GET_INSTRUCTIONS_RAN>(str.as_ptr().addr() as u32);
    }
}

#[inline(always)]
pub fn print_str(str: &str) {
    for char in str.chars() {
        print_char(char);
    }
}

#[inline(always)]
pub fn print_char(char: char) {
    unsafe {
        syscall_s_v::<PRINT_CHAR>(char as u32);
    }
}

#[inline(always)]
pub fn sleep_ms(ms: u32) {
    unsafe {
        syscall_s_v::<SLEEP_MS>(ms);
    }
}

#[inline(always)]
pub fn sleep_d_ms(ms: u32) {
    unsafe {
        syscall_s_v::<SLEEP_D_MS>(ms);
    }
}

#[inline(always)]
pub fn current_time_nanos() -> u64 {
    unsafe { syscall_v_d::<CURRENT_TIME_NANOS>() }
}

pub fn is_key_pressed(char: char) -> bool {
    unsafe { syscall_s_s::<IS_KEY_PRESSED>(char as u32) != 0 }
}

// #[inline(always)]
// pub fn read_i32() -> i32 {
//     unsafe { syscall_0_1::<5>() as i32 }
// }

pub fn rand_range(min: i32, max: i32) -> i32 {
    unsafe { syscall_ss_s::<GENERATE_THREAD_RANDOM_NUMBER>(min as u32, max as u32) as i32 }
}

// #[inline(always)]
// pub fn sleep_delta_mills(mills: u32) {
//     unsafe {
//         syscall_1_0::<106>(mills);
//     }
// }

// #[inline(always)]
// pub fn sleep_mills(mills: u32) {
//     unsafe {
//         syscall_1_0::<105>(mills);
//     }
// }

// #[inline(always)]
// pub fn get_micros() -> u64 {
//     unsafe { syscall_0_2_s::<108>() }
// }

// #[inline(always)]
// pub fn get_nanos() -> u64 {
//     unsafe { syscall_0_2_s::<109>() }
// }

//--------------------------------------------------------------
/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_v_v<const CALL_ID: u32>() {
    asm!(
        "syscall {0}",
        const(CALL_ID),
    )
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_s_v<const CALL_ID: u32>(arg1: u32) {
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
    )
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_v_s<const CALL_ID: u32>() -> u32 {
    let ret1;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        out("$2") ret1,
    );
    ret1
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_v_ss<const CALL_ID: u32>() -> (u32, u32) {
    let ret1;
    let ret2;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        out("$2") ret1,
        out("$3") ret2,
    );
    (ret1, ret2)
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_v_d<const CALL_ID: u32>() -> u64 {
    let v0: u32;
    let v1: u32;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        lateout("$2") v0,
        lateout("$3") v1,
    );
    ((v0 as u64) << 32) | (v1 as u64)
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_s_s<const CALL_ID: u32>(arg1: u32) -> u32 {
    let ret1;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        out("$2") ret1,
    );
    ret1
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_s_ss<const CALL_ID: u32>(arg1: u32) -> (u32, u32) {
    let ret1;
    let ret2;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        out("$2") ret1,
        out("$3") ret2,
    );
    (ret1, ret2)
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_s_d<const CALL_ID: u32>(arg1: u32) -> u64 {
    let v0: u32;
    let v1: u32;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        lateout("$2") v0,
        lateout("$3") v1,
    );
    ((v0 as u64) << 32) | (v1 as u64)
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_ss_v<const CALL_ID: u32>(arg1: u32, arg2: u32) {
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        in("$5") arg2,
    );
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_d_v<const CALL_ID: u32>(arg1: u64) {
    let a0 = (arg1 >> 32) as u32;
    let a1 = arg1 as u32;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") a0,
        in("$5") a1,
    );
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_d_s<const CALL_ID: u32>(arg1: u64) -> u32 {
    let a0 = (arg1 >> 32) as u32;
    let a1 = arg1 as u32;
    let ret;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") a0,
        in("$5") a1,
        out("$2") ret,
    );
    ret
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_sss_v<const CALL_ID: u32>(arg1: u32, arg2: u32, arg3: u32) {
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
    );
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_sss_s<const CALL_ID: u32>(arg1: u32, arg2: u32, arg3: u32) -> u32 {
    let ret1;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        out("$2") ret1,
    );
    ret1
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_sss_ss<const CALL_ID: u32>(arg1: u32, arg2: u32, arg3: u32) -> (u32, u32) {
    let ret1;
    let ret2;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        in("$5") arg2,
        in("$6") arg3,
        out("$2") ret1,
        out("$3") ret2,
    );
    (ret1, ret2)
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_ds_v<const CALL_ID: u32>(arg1: u64, arg2: u32) {
    let a0 = (arg1 >> 32) as u32;
    let a1 = arg1 as u32;
    let a2 = arg2;
    // let arg_2 = (arg2 >> 32) as u32;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") a0,
        in("$5") a1,
        in("$6") a2,
    );
}

/// # Safety
///
/// If you have to read this then you shouldnt be using this. This is a raw System Call, using it
/// incorrectly can break pretty much anything.
#[inline(always)]
pub unsafe fn syscall_ss_s<const CALL_ID: u32>(arg1: u32, arg2: u32) -> u32 {
    let ret1;
    asm!(
        "syscall {0}",
        const(CALL_ID),
        in("$4") arg1,
        in("$5") arg2,
        out("$2") ret1,
    );
    ret1
}
