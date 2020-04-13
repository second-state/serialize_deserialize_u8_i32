# Serialize & deserialize back and forth between u8 and i32
A Rust library that safely converts, back and forward, between u8 and i32

# Example usage (a recipe which also uses serde and bincode)
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
}
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
println!("As i32: {:?}", encoded_i32);
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

# Serializing u8 to i32 explicitly (a recipe that does not use serde or bincode)
If you are interested in using a highly performant data model with a minimum of dependencies, please consider the following.
As you can see from the examples above, this library can facilitate the storage and retrieval of high-level complex data types in a generic way. 
Naturally, this is very simple and easy to use.
**You can, however**, go a step further and explicitly encode your data to i32 yourself, ahead of time. Essentially what this means is, instead of creating a generic representation of your data, you can crack your PhotonImage object open (ahead of time) to serialize and store each internal part separately. 

Why would you want to do this? 

So that you can build your intense computation to be more effieicnt. Let me explain.
If you [store](https://github.com/second-state/specs/blob/master/storage_interface.md#store-a-custom-struct) your data as a high-leve data type, the application that uses it will have to unpack it. The unpacking is an overhead that your execution may not want. In addition, the inpacking requires dependencies like serde and bincode. 
You can still store and load the high level object. Just do that in a different Rust/Wasm executable.
If you want maximum efficiency and you have data that qualifies i.e. an array of pixels (`[u8]`) you can store these in such a way that the Wasm VM can natively process them (without any serde & bincode overhead)
Here is an example of the discrete application which would just perform pixel processing, with minimal overheads
Cargo.toml
```rust, ignore
[dependencies]
serialize_deserialize_u8_i32 = "^0.1"
rust_storage_interface_library = "^0.1"
```
Rust/Wasm pixel processing function
```rust, ignore
use serialize_deserialize_u8_i32::s_d_u8_i32;
use rust_storage_interface_library::ssvm_storage;
```
Takes the i32 storage key for a specific image, converts the image and returns a new storage key to the newly generated (solarized) image
```
#[no_mangle]
pub extern fn solarize_the_pixels(_orig_image_location: i32) -> i32 {
    // Load your data from the storage layer (u8 pixels are stored at a compression rate of 3:1)
    let i32_vec: Vec<i32> = ssvm_storage::load::load_as_i32_vector(storage_key);
    // Quickly convert it to pixel data
    let mut individual_pixels: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(i32_vec);
    // Process each pixel directly inside the VM
    for pixel in individual_pixels.iter_mut() {
        if 200 as i32 - *pixel as i32 > 0 {
            *pixel = 200 - *pixel;
        }
    }
    // Pack the u8 pixels back into i32s (compressing 3:1)
    let new_encoded_image: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(individual_pixels);
    // Save the solarized image to the storage location and retrieve its storage key
    let new_image_storage_key: i32 = ssvm_storage::store::store_as_i32_vector(new_encoded_image);
    // Pass the storage key of the solarized image back to the calling code
    new_image_storage_key
}
```
