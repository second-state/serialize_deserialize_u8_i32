//! # Serializing high-level data types 
//! The following section documents the process of serializing complex high-level data types to i32. 
//! 
//! Using the following method, you can serialize your own custom data structures to i32. For example, want to serialize and deserialize a custom PhotonImage struct? Here's how ...
//! Add serialize_deserialize_u8_i32 dependency in your Cargo.toml file
//! (please note that we have also added serde and bincode for this usage example)
//!
//! ```rust, ignore
//! [dependencies]
//! serialize_deserialize_u8_i32 = "^0.1"
//! serde = { version = "1.0.104", features = ["derive"] }
//! bincode = "1.2.1"
//! ```
//! Add the following code to your application
//! ```rust, ignore
//! use serde::{Deserialize, Serialize};
//! use serialize_deserialize_u8_i32::s_d_u8_i32;
//! use bincode;
//!```
//! Create a high level data type and then serialize and deserialize...
//! ```rust, ignore
//! // Create a high level custom struct
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct PhotonImage {
//!     raw_pixels: Vec<u8>,
//!     width: u32,
//!     height: u32,
//! }
//!
//! // Serialize and deserialize
//! ```rust, ignore
//! fn main() {
//!     // Imlement the struct with data
//!     let photon_image = PhotonImage {
//!         raw_pixels: vec![
//!             134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126,
//!             125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125,
//!             132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110,
//!             255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255,
//!         ],
//!         width: 4,
//!         height: 4,
//!     };
//!     println!("PhotonImage: {:?}", photon_image);
//!     /*
//!     PhotonImage: PhotonImage { raw_pixels: [134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255], width: 4, height: 4 }
//!     */
//!
//!     // Serialize that to standard u8
//!     let encoded_u8: Vec<u8> = bincode::serialize(&photon_image).unwrap();
//!     println!("As u8: {:?}", encoded_u8);
//!     /*
//!     As u8: [64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
//!     */
//!
//!     // Serialize that to i32
//!     let encoded_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_u8);
//!     println!("As i32: {:?}", encoded_i32);
//!     /*
//!     As i32: [1064000000, 1000000000, 1000000134, 1122131255, 1131131139, 1255135134, 1137255138, 1134130255, 1126125119, 1255131134, 1129255137, 1134132255, 1130126130, 1255132125, 1132255122, 1142129255, 1134135128, 1255138120, 1125255125, 1134110255, 1121122137, 1255141140, 1141255125, 1144120255, 1004000000, 1000004000, 2000000000]
//!     */
//!
//!     // Deserialize back to u8
//!     let encoded_u8_again: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(encoded_i32);
//!     println!("As u8 again: {:?}", encoded_u8_again);
//!     /*
//!     As u8 again: [64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
//!     */
//!
//!     // Deserialize back to Rust
//!     let decoded: PhotonImage = bincode::deserialize(&encoded_u8_again[..]).unwrap();
//!     println!("As PhotonImage again: {:?}", decoded);
//!     /*
//!     As PhotonImage again: PhotonImage { raw_pixels: [134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255], width: 4, height: 4 }
//!     */
//!```
//!
//! # Serializing u8 to i32 explicitly
//! If you are interested in using a highly performant data model with a minimum of dependencies, please consider the following.
//! As you can see from the examples above, this library can facilitate the storage and retrieval of high-level complex data types in a generic way. 
//! Naturally, this is very simple and easy to use.
//! **You can, however**, go a step further and explicitly encode your data to i32 yourself, ahead of time. Essentially what this means is, instead of creating a generic representation of your data, you can crack your PhotonImage object open (ahead of time) to serialize and store each internal part separately. 
//! 
//! Why would you want to do this? 
//! 
//! So that you can build your intense computation to be more effieicnt. Let me explain.
//! If you [store](https://github.com/second-state/specs/blob/master/storage_interface.md#store-a-custom-struct) your data as a high-leve data type, the application that uses it will have to unpack it. The unpacking is an overhead that your execution may not want. In addition, the inpacking requires dependencies like serde and bincode. 
//! You can still store and load the high level object. Just do that in a different Rust/Wasm executable.
//! If you want maximum efficiency and you have data that qualifies i.e. an array of pixels (`[u8]`) you can store these in such a way that the Wasm VM can natively process them (without any serde & bincode overhead)
//! Here is an example of the discrete application which would just perform pixel processing, with minimal overheads
//! Cargo.toml
//! ```rust, ignore
//! [dependencies]
//! serialize_deserialize_u8_i32 = "^0.1"
//! rust_storage_interface_library = "^0.1"
//! ```
//! Rust/Wasm pixel processing function
//! ```rust, ignore
//! use serialize_deserialize_u8_i32::s_d_u8_i32;
//! use rust_storage_interface_library::ssvm_storage;
//! // Takes the i32 storage key for a specific image, converts the image and returns a new storage key to the newly generated (solarized) image
//! #[no_mangle]
//! pub extern fn solarize_the_pixels(_orig_image_location: i32) -> i32 {
//!     // Load your data from the storage layer (u8 pixels are stored at a compression rate of 3:1)
//!     let i32_vec: Vec<i32> = ssvm_storage::load::load_as_i32_vector(storage_key);
//!     // Quickly convert it to pixel data
//!     let mut individual_pixels: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(i32_vec);
//!     // Process each pixel directly inside the VM
//!     for pixel in individual_pixels.iter_mut() {
//!         if 200 as i32 - *pixel as i32 > 0 {
//!             *pixel = 200 - *pixel;
//!         }
//!     }
//!     // Pack the u8 pixels back into i32s (compressing 3:1)
//!     let new_encoded_image: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(individual_pixels);
//!     // Save the solarized image to the storage location and retrieve its storage key
//!     let new_image_storage_key: i32 = ssvm_storage::store::store_as_i32_vector(new_encoded_image);
//!     // Pass the storage key of the solarized image back to the calling code
//!     new_image_storage_key
//! }
//! ```


pub mod s_d_u8_i32 {
    use std::convert::TryInto;

    pub fn exceeding_max_i32_threshold(_num: u64) -> bool {
        let max: u64 = i32::max_value().try_into().unwrap();
        if _num > max {
            true
        } else {
            false
        }
    }

    pub fn count_vec_items_left(_vec: &Vec<u8>) -> u64 {
        let items_left: u64 = _vec.len().try_into().unwrap();
        items_left
    }

    pub fn flush_value_to_zero(_value: u64, _position: u64, _size: u64) -> u64 {
        let new_value: u64 = _value
            - ((_value % (10_u64.pow(_position.try_into().unwrap())))
                - (_value % (10_u64.pow((_position - _size).try_into().unwrap()))));
        new_value
    }

    pub fn insert_value_at_position(
        _value: u64,
        _single_value: u64,
        _position: u64,
        _size: u64,
    ) -> u64 {
        // buffer up the single value to equal size i.e. turn 55 (two digits) into 055 (three digits) where the size is 3 etc.
        let mut string_single_value = _single_value.to_string();
        while string_single_value.len() < (_size as u64).try_into().unwrap() {
            string_single_value = "0".to_owned() + &string_single_value;
        }
        let new_single_value: u64 = string_single_value.parse::<u64>().unwrap();
        let zeroed_value: u64 = flush_value_to_zero(_value, _position, _size);
        let new_value: u64 =
            zeroed_value + new_single_value * (10_u64.pow((_position - _size).try_into().unwrap()));
        new_value
    }

    pub fn access_value(_value: u64, _position: u64, _size: u64) -> u64 {
        let _mode: u64 = ((_value % (10_u64.pow(_position.try_into().unwrap())))
            - (_value % (10_u64.pow((_position - _size).try_into().unwrap()))))
            / (10_u64.pow((_position - _size).try_into().unwrap()));
        _mode
    }

    pub fn serialize_u8_to_i32(u8_data: Vec<u8>) -> Vec<i32> {
        let mut vec_of_i32s: Vec<i32> = Vec::new();
        // Test to see if there are too many i32s to store (we need to store the number of i32s in the first i32 so this can not exceed 2147483647)
        if exceeding_max_i32_threshold(count_vec_items_left(&u8_data).into()) == false {
            let items_left: u64 = count_vec_items_left(&u8_data).try_into().unwrap();
            // Begin processing all of the data into i32s
            let batches_left: u64 = items_left / 3;
            //println!("Batches to process: {:?}", batches_left);
            let last_batch_count: u64 = items_left % 3;
            if batches_left >= 1 {
                for i in 1..=batches_left {
                    //println!("Processing: {:?}", i);
                    // Create a placeholder i32
                    let mut single_value_for_i32_vec: u64 = 1000000000;
                    // Vec position setup
                    let mut n: u64 = 0;
                    if i == 2 {
                        n = 3;
                    } else if i >= 3 {
                        n = i + (i - 1) + (i - 2);
                    }
                    let one: u64 = (*u8_data.get(n as usize).unwrap()).into();
                    //println!("One: {:?}", one);
                    let two: u64 = (*u8_data.get(n as usize + 1).unwrap()).into();
                    //println!("Two: {:?}", two);
                    let three: u64 = (*u8_data.get(n as usize + 2).unwrap()).into();
                    //println!("Three: {:?}", three);
                    //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 9, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, one as u64, 9, 3);
                    //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 6, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, two as u64, 6, 3);
                    //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, three as u64, 3, 3);
                    vec_of_i32s.push(single_value_for_i32_vec.try_into().unwrap());
                }
            }
            let mut index: usize = 0;
            if batches_left == 0 {
                index = 0;
            } else if batches_left == 1 {
                index = 3;
            } else if batches_left >= 2 {
                index = (batches_left as usize + 3) + (batches_left as usize - 1) + (batches_left as usize - 2);
            }
            // See how many items we have left in the serialised Vec<u8>
            if last_batch_count == 1 {
                // Create a placeholder i32
                let mut single_value_for_i32_vec: u64 = 1000000000;

                let one: u64 = (*u8_data.get(index).unwrap()).into();
                //println!("One: {:?}", one);
                //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                single_value_for_i32_vec =
                    insert_value_at_position(single_value_for_i32_vec, one as u64, 3, 3);
                // Set the indicator to 3
                single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 10, 1);
                // A single u8 stored in a single i32 will have a prefix of 3 - this is a code used in encoding/decoding
                single_value_for_i32_vec =
                    insert_value_at_position(single_value_for_i32_vec, 0, 10, 1);
                // Push this new i32 to the vec_of_i32s
                vec_of_i32s.push(single_value_for_i32_vec.try_into().unwrap());
            }
            if last_batch_count == 2 {
                // Create a placeholder i32
                let mut single_value_for_i32_vec: u64 = 1000000000;
                let one: u64 = (*u8_data.get(index).unwrap()).into();
                //println!("One: {:?}", one);
                let two: u64 = (*u8_data.get(index + 1).unwrap()).into();
                //println!("Two: {:?}", two);
                //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 6, 3);
                single_value_for_i32_vec =
                    insert_value_at_position(single_value_for_i32_vec, one as u64, 6, 3);
                //single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                single_value_for_i32_vec =
                    insert_value_at_position(single_value_for_i32_vec, two as u64, 3, 3);
                // Set the indicator to 2
                single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 10, 1);
                // When two u8s are stored in a single i32 it will have a prefix of 2 - this is a code used in encoding/decoding
                single_value_for_i32_vec =
                    insert_value_at_position(single_value_for_i32_vec, 2, 10, 1);
                // Push this new i32 to the vec_of_i32s
                vec_of_i32s.push(single_value_for_i32_vec.try_into().unwrap());
            }
        }
        vec_of_i32s
    }

    pub fn deserialize_i32_to_u8(_i32_data: Vec<i32>) -> Vec<u8> {
        let mut vec_of_u8s: Vec<u8> = Vec::new();
        for single_i32_from_vec in _i32_data {
            //println!("Processing: {:?}", single_i32_from_vec);
            let mode: u64 = access_value(single_i32_from_vec as u64, 10, 1);
            //println!("Mode: {:?}", mode);
            if mode == 1 {
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 9, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 6, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
            if mode == 2 {
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 6, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
            // It is impossible for the other cases (which start with 1 or 2) to be less than or equal to 255. This will still work even if the 0000000000 -> 0000000255 gets appended to 0 -> 255
            if mode == 0 || single_i32_from_vec <= 255 {
                vec_of_u8s.push(
                    access_value(single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
        }
        vec_of_u8s
    }
}

#[cfg(test)]
mod tests {
    use super::s_d_u8_i32;
    #[test]
    fn test_flush_3_3_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1000000123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1000000999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1000123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1000999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 6, 3);
        assert_eq!(v, 1999000999);
    }
    #[test]
    fn test_flush_9_3_999() {
        let _test_single_value_for_i32_vec_000: u64 = 1999000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 9, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_access_3_3_1() {
        let _test_single_value_for_i32_vec_000: u64 = 1009010011;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_000, 10, 1);
        assert_eq!(v, 1);
    }
    #[test]
    fn test_access_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123123;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_123, 3, 3);
        assert_eq!(v, 123);
    }
    #[test]
    fn test_access_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_999, 3, 3);
        assert_eq!(v, 999);
    }
    #[test]
    fn test_insert_3_3_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let _single_val: u64 = 000;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_000,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_insert_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123000;
        let _single_val: u64 = 123;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_123,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1123123123);
    }
    #[test]
    fn test_insert_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999009;
        let _single_val: u64 = 999;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_999,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1999999999);
    }
    #[test]
    fn test_insert_9_9_111() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let _single_val: u64 = 111;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_999,
            _single_val,
            9,
            3,
        );
        assert_eq!(v, 1111999999);
    }
    #[test]
    fn test_i32_threshold_over() {
        let number: u64 = 2147483648;
        let b = s_d_u8_i32::exceeding_max_i32_threshold(number);
        assert_eq!(b, true);
    }
    #[test]
    fn test_i32_threshold_under() {
        let number: u64 = 2147483647;
        let b = s_d_u8_i32::exceeding_max_i32_threshold(number);
        assert_eq!(b, false);
    }
    #[test]
    fn test_count_vec_items_left() {
        let mut vec = Vec::with_capacity(10);
        for i in 0..10 {
            vec.push(i);
        }
        let items_left: u64 = s_d_u8_i32::count_vec_items_left(&vec);
        assert_eq!(items_left, 10);
    }
    #[test]
    fn test_serialize_u8_to_i32_one() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 1..=3 {
            vec.push(i);
        }
        // Creates
        // [1, 2, 3]

        // Expected result
        // [1001002003]
        let mut a: Vec<i32> = Vec::new();
        a.push(1001002003);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_serialize_u8_to_i32_two() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        // Creates
        // [1, 2, 3, 4, 5, 6]

        // Expected result
        // [1001002003, 1004005006]
        let mut a: Vec<i32> = Vec::new();
        a.push(1001002003);
        a.push(1004005006);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_serialize_u8_to_i32_three() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 99..=105 {
            vec.push(i);
        }
        // Creates
        // [99, 100, 101, 102, 103, 104, 105]

        // Expected result
        // [1099100101, 1102103104, 0000000105]
        let mut a: Vec<i32> = Vec::new();
        a.push(1099100101);
        a.push(1102103104);
        a.push(0000000105);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_four() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 99..=106 {
            vec.push(i);
        }
        // Creates
        // [99, 100, 101, 102, 103, 104, 105, 106]

        // Expected result
        // [1099100101, 1102103104, 2000105106]
        let mut a: Vec<i32> = Vec::new();
        a.push(1099100101);
        a.push(1102103104);
        a.push(2000105106);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_five() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 9..=16 {
            vec.push(i);
        }
        // Creates
        // [9, 10, 11, 12, 13, 14, 15, 16]

        // Expected result
        // [1009010011, 1012013014, 2000015016]
        let mut a: Vec<i32> = Vec::new();
        a.push(1009010011);
        a.push(1012013014);
        a.push(2000015016);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_six() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        // Creates
        // [1]

        // Expected result
        // [0000000001]
        let mut a: Vec<i32> = Vec::new();
        a.push(0000000001);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_deserialize_i32_to_u8_one() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1009010011);
        vec.push(1012013014);
        vec.push(2000015016);
        println!("vec: {:?}", vec);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(9);
        a.push(10);
        a.push(11);
        a.push(12);
        a.push(13);
        a.push(14);
        a.push(15);
        a.push(16);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8( vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 8);
    }

    #[test]
    fn test_deserialize_i32_to_u8_two() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(0000000001);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(1);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8( vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two thatboth match - success
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_deserialize_i32_to_u8_three() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1009010011);
        vec.push(1012013014);
        println!("vec: {:?}", vec);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(9);
        a.push(10);
        a.push(11);
        a.push(12);
        a.push(13);
        a.push(14);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 6);
    }
}