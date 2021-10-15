#Basic Data Types
##integers
    * u8, i8 - u means unsignend; 8 = the size in bits
    * u16,i16
    * u32, i32
    * u64, i64
    * u128, i128

##float
    * f32, f64

##bool
    * bool -> one bit

##size
    * usize, isize (CPU architecture specific 32 or 64 bits)

##char
    * holds single character
    * unicode values
    * 4 bytes -> less efficient for ASCII where 1 byte is ok

#functions
fn the_name(arg:i32) -> i32{
    return arg + 42;
}

OR

fn the_name(arg:i32) -> i32{
    42
}