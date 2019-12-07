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
in Windows 8.1 and there's not really a good way to just get the version now, not that *that* function was very reliable.
