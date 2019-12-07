# nt_version
Queries the major, minor and build version of Windows (NT) efficiently with usage of undocumented ntdll functions.


## Usage
It only has one function: [get](https://www.google.com/), and it's recommended you use it explicitly.

```rust
fn main() {
    let (major, minor, build) = nt_version::get();
    println!("NT Version v{}.{}.{}", major, minor, build);
}
```
