//! Queries the major, minor and build version of Windows (NT) efficiently
//! with usage of undocumented NTDLL functions. **This crate is no_std.**
//!
//! It only has one function: [get](fn.get.html),
//! and it's recommended you use it explicitly like `nt_version::get()` because of this.
//!
//!
//! ## Build Error?
//! If building fails with a linker error, you're missing `ntdll.lib` from your system.
//! It doesn't come on older versions of Windows with the SDK and you need to install the DDK.
//!
//! You can alternatively enable the **fallback** feature which queries the function pointer at runtime.

#![no_std]

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
/// The versions correspond to this table (transcribed from [here](http://www.geoffchappell.com/studies/windows/win32/ntdll/history/index.htm)):
///
/// | Version | Windows | NT |
/// | ------- | ------- | -- |
/// | 3.51 | | Windows NT 3.51 |
/// | 4.0 | Windows 95 | Windows NT 4.0 |
/// | 4.10 | Windows 98 | |
/// | 4.90 | Windows Me | |
/// | 5.0 | | Windows 2000 |
/// | 5.1 | | Windows XP |
/// | 5.2 | | Windows Server 2003 |
/// | 6.0 | | Windows Vista / Windows Server 2008 |
/// | 6.1 | | Windows 7 / Windows Server 2008 R2 |
/// | 6.2 | | Windows 8 |
/// | 6.3 | | Windows 8.1 |
/// | 10.0 | | Windows 10 |
pub fn get() -> (u32, u32, u32) {
    let (mut major, mut minor, mut build) = (0u32, 0u32, 0u32);
    unsafe {
        internal::RtlGetNtVersionNumbers(&mut major as _, &mut minor as _, &mut build as _);
    }
    (major, minor, build)
}
