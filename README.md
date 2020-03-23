# Serialize & deserialize back and forth between u8 and i32
A Rust library that safely converts, back and forward, between u8 and i32

# Example usage
Consider you have the following custom image object, in your code (a struct with raw pixels as well as width and height).

```
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}
```
Your `main.rs` might look something like this
```
use serde::{Deserialize, Serialize};
fn main() {
    let photon_image = PhotonImage {
        raw_pixels: vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126,
            125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125,
            132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110,
            255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255,
        ],
        width: 4,
        height: 4,
    };
```
Your dependencies might look like this
```
[dependencies]
serde = { version = "1.0.104", features = ["derive"] }
```

# Why serialize as i32 integers?
There are applications like SecondState's [Rust Storage Interface Library](https://github.com/second-state/rust_storage_interface_library), that let you store and load objects as i32. This crate allows you to serialize your data to i32 so that you can take advantge of these storage opportunities.

# Serializing
Add `bincode = "^1.2"` and `serialize_deserialize_u8_i32 = "^0.1"` to your dependencies. They will not look like this.
```
[dependencies]
bincode = "^1.2"
serialize_deserialize_u8_i32 = "^0.1"
serde = { version = "1.0.104", features = ["derive"] }
```
Add the following code to your main function to serialize to `u8`
```
let mut encoded: Vec<u8> = bincode::serialize(&photon_image).unwrap();
```
This will result in the following data structure
```bash
[64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
```
The also add the following code to your main function to serialize to `i32`

#TODO import library via `use` statement

#TODO demonstrate how to call library serialize

#TODO show data output

# Why deserialize i32 to u8?
This crate also allows you to load your i32 data from SecondState's [Rust Storage Interface Library](https://github.com/second-state/rust_storage_interface_library) and turn it back into your original high level Rust object.

# Serializing
#TODO demonstrate how to call library deserialize

#TODO show data output
