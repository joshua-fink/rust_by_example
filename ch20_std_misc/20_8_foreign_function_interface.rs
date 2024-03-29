// Foreign Function Interface

// Rust provides FFI to C libraries, must be declared in extern block annotated with `[link]` attribute with name of foreign library

use std::fmt;

// extern block links to the libm library
#[cfg(target_family = "windows")]
#[link(name = "msvcrt")]
extern {
    //foreign function
    // computes sqrt of single precision num
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}
#[cfg(target_family="unix")]
#[link(name = "m")]
extern {
    // same as windows version
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}

// Since calling functions is considered unsafe, common to write safe wrappers around them
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling foreign function is unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    println!("cos({:?}) = {:?}", z, cos(z));
}

// Minimal impl of single precision complex nums
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}