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

## Serializing
Add `bincode = "^1.2"` and `serialize_deserialize_u8_i32 = "^0.1"` to your dependencies. They will not look like this.
```
[dependencies]
bincode = "^1.2"
serialize_deserialize_u8_i32 = "^0.1"
serde = { version = "1.0.104", features = ["derive"] }
```
Then also add the following code to your main function to serialize to `i32`
```
use serde::{Deserialize, Serialize};
use serialize_deserialize_u8_i32::s_d_u8_i32;
use bincode;
```
Add the following code to your main function to serialize to `u8`
```
let encoded_u8: Vec<u8> = bincode::serialize(&photon_image).unwrap();
```
This will result in the following data structure
```bash
[64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
```
Serialize to i32
```
// Serialize that to i32
let encoded_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_u8);
println!("As i32: {:?}", encoded_i32)
```
Results in the following
```
As i32: [1064000000, 1000000000, 1000000134, 1122131255, 1131131139, 1255135134, 1137255138, 1134130255, 1126125119, 1255131134, 1129255137, 1134132255, 1130126130, 1255132125, 1132255122, 1142129255, 1134135128, 1255138120, 1125255125, 1134110255, 1121122137, 1255141140, 1141255125, 1144120255, 1004000000, 1000004000, 2000000000]
```

# Why deserialize i32 to u8?
This crate also allows you to load your i32 data from SecondState's [Rust Storage Interface Library](https://github.com/second-state/rust_storage_interface_library) and turn it back into your original high level Rust object.

## Deserializing
```
// Deserialize back to u8
let encoded_u8_again: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(encoded_i32);
println!("As u8 again: {:?}", encoded_u8_again);
```
Results in the following 
```
As u8 again: [64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
```

Deserialize back to Rust 
```
let decoded: PhotonImage = bincode::deserialize(&encoded_u8_again[..]).unwrap();
println!("As PhotonImage again: {:?}", decoded);
```
Results in the following
```
As PhotonImage again: PhotonImage { raw_pixels: [134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255], width: 4, height: 4 }
```
