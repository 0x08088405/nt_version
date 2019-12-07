//! Queries the major, minor and build version of Windows (NT) efficiently
//! with usage of undocumented NTDLL functions.
//!
//! It only has one function: [get](fn.get.html),
//! and it's recommended you use it explicitly like `nt_version::get()` because of this.

#[cfg(not(target_os = "windows"))]
compile_error!("This crate is for querying Windows, but the target isn't Windows.");

#[cfg(not(feature = "fallback"))]
mod internal {
    #[link(name = "ntdll")]
    extern "C" {
        pub fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32);
    }
}

#[cfg(feature = "fallback")]
mod internal {
    #![allow(non_snake_case)]

    use std::{mem::transmute, ptr};
    use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

    static mut NTDLL_FUNCTION: *const () = ptr::null();
    pub unsafe fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32) {
        if NTDLL_FUNCTION.is_null() {
            NTDLL_FUNCTION = GetProcAddress(
                GetModuleHandleA(b"ntdll.dll\0".as_ptr() as *const _),
                b"RtlGetNtVersionNumbers\0".as_ptr() as *const _,
            ) as *const _;
        }
        transmute::<_, extern "stdcall" fn(*mut u32, *mut u32, *mut u32)>(NTDLL_FUNCTION)(
            major, minor, build,
        );
    }
}

/// Queries the (major, minor, build) version of the Windows NT system.
pub fn get() -> (u32, u32, u32) {
    let (mut major, mut minor, mut build) = (0u32, 0u32, 0u32);
    unsafe {
        internal::RtlGetNtVersionNumbers(&mut major as _, &mut minor as _, &mut build as _);
    }
    (major, minor, build)
}
