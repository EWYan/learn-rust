// this extern block links to the libc library
#[link(name = "c")]
extern {
    // this is a foreign function
    fn abs(z: i32) -> i32;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
fn abs_ffi(z: i32) -> i32 {
    unsafe { abs(z) }
}

fn main() {
    let z = -123;

    println!("abs({:?}) = {:?}", z, abs_ffi(z));
}

//abs(-123) = 123
