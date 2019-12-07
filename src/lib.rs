//! Queries the major, minor and build version of Windows (NT) efficiently
//! with usage of undocumented ntdll functions.
//!
//! It only has one function: [get](fn.get.html),
//! and it's recommended you use it explicitly like `nt_version::get()` because of this.

#[link(name = "ntdll")]
extern "C" {
    fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32);
}

/// Queries the (major, minor, build) version of the Windows NT system.
pub fn get() -> (u32, u32, u32) {
    let (mut major, mut minor, mut build) = (0u32, 0u32, 0u32);
    unsafe {
        RtlGetNtVersionNumbers(&mut major as _, &mut minor as _, &mut build as _);
    }
    (major, minor, build)
}
