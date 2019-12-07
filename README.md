[![Build Status](https://ci.appveyor.com/api/projects/status/bqhc99s3k0bm1qv3?svg=true)](https://ci.appveyor.com/project/viri/nt-version)

# nt_version
Queries the major, minor, and build version of Windows (NT) efficiently with usage of undocumented NTDLL functions.
**Needs a minimum version of NT 5.1 (Windows XP or above)**
```toml
nt_version = "0.1"
```
If building fails with a linker error, you're missing `ntdll.lib` from your system.
It doesn't come on older versions of Windows with the SDK and you need to install the DDK.

Alternatively, enable the fallback feature which queries the function pointer at runtime (but is slower):
```toml
nt_version = { version = "0.1", features = ["fallback"] }
```

## Usage
It only has one function: [get](https://www.google.com/), and it's recommended you use it explicitly:

```rust
fn main() {
    let (major, minor, build) = nt_version::get();
    println!("NT Version v{}.{}.{}", major, minor, build);
}
```
This returns the NTDLL version, which has [this numbering system](http://www.geoffchappell.com/studies/windows/win32/ntdll/history/index.htm).

## Why?
Microsoft deprecated [GetVersionEx](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getversionexa)
back in Windows 8.1 in favour of functions such as "[IsWindows8Point1OrGreater](https://docs.microsoft.com/en-us/windows/win32/sysinfo/version-helper-apis)" and there's not really a good way to just get an OS version now. You can still use [NetWkstaGetInfo](https://docs.microsoft.com/en-us/windows/win32/api/lmwksta/nf-lmwksta-netwkstagetinfo) from `lmwksta.h`
if you just want the OS version without any build numbers, but it's much slower and allocates a [WKSTA_INFO_100](https://docs.microsoft.com/en-us/windows/win32/api/lmwksta/ns-lmwksta-wksta_info_100) chunk which you have to deallocate yourself,
containing excess info your like PC name as a windows wide-string. Not very ideal.
