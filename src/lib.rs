mod s_d_u8_i32 {
    use std::convert::TryInto;

    pub fn exceeding_max_i32_threshold(_num: u64) -> bool {
        let max: u64 = i32::max_value().try_into().unwrap();
        println!("Maximum allowable integer is {:?}", max);
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
        while string_single_value.len() < _size as usize {
            string_single_value = "0".to_owned() + &string_single_value;
        }
        let new_single_value: u64 = string_single_value.parse::<u64>().unwrap();
        let zeroed_value: u64 = flush_value_to_zero(_value, _position, _size);
        let new_value: u64 = zeroed_value + new_single_value * (10_u64.pow((_position - _size).try_into().unwrap()));
        new_value
    }

    pub fn access_value(_value: u64, _position: u64, _size: u64) -> u64 {
        let _mode: u64 = (_value % (10_u64.pow(_position.try_into().unwrap())))
            - (_value % (10_u64.pow((_position - _size).try_into().unwrap())))
                / (10_u64.pow((_position - _size).try_into().unwrap()));
        _mode
    }
}

#[cfg(test)]
mod tests {
    use super::s_d_u8_i32;
    #[test]
    fn test_flush_3_3_000() {
        let _test_single_i32_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_000, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_123() {
        let _test_single_i32_123: u64 = 1000000123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_123, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_999() {
        let _test_single_i32_999: u64 = 1000000999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_999, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_000() {
        let _test_single_i32_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_000, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_123() {
        let _test_single_i32_123: u64 = 1000123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_123, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_999() {
        let _test_single_i32_999: u64 = 1000999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_999, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_000() {
        let _test_single_i32_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_000, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_3_999() {
        let _test_single_i32_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_999, 6, 3);
        assert_eq!(v, 1999000999);
    }
    #[test]
    fn test_flush_9_3_999() {
        let _test_single_i32_000: u64 = 1999000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_000, 9, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_123() {
        let _test_single_i32_123: u64 = 1123123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_123, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_999() {
        let _test_single_i32_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_i32_999, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_access_3_3_000() {
        let _test_single_i32_000: u64 = 1000000000;
        let v = s_d_u8_i32::access_value(_test_single_i32_000, 3, 3);
        assert_eq!(v, 000);
    }
    #[test]
    fn test_access_3_3_123() {
        let _test_single_i32_123: u64 = 1123123123;
        let v = s_d_u8_i32::access_value(_test_single_i32_123, 3, 3);
        assert_eq!(v, 123);
    }
    #[test]
    fn test_access_3_3_999() {
        let _test_single_i32_999: u64 = 1999999999;
        let v = s_d_u8_i32::access_value(_test_single_i32_999, 3, 3);
        assert_eq!(v, 999);
    }
    #[test]
    fn test_insert_3_3_000() {
        let _test_single_i32_000: u64 = 1000000000;
        let _single_val: u64 = 000;
        let v = s_d_u8_i32::insert_value_at_position(_test_single_i32_000, _single_val, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_insert_3_3_123() {
        let _test_single_i32_123: u64 = 1123123000;
        let _single_val: u64 = 123;
        let v = s_d_u8_i32::insert_value_at_position(_test_single_i32_123, _single_val, 3, 3);
        assert_eq!(v, 1123123123);
    }
    #[test]
    fn test_insert_3_3_999() {
        let _test_single_i32_999: u64 = 1999999009;
        let _single_val: u64 = 999;
        let v = s_d_u8_i32::insert_value_at_position(_test_single_i32_999, _single_val, 3, 3);
        assert_eq!(v, 1999999999);
    }
    #[test]
    fn test_insert_9_9_111() {
        let _test_single_i32_999: u64 = 1999999999;
        let _single_val: u64 = 111;
        let v = s_d_u8_i32::insert_value_at_position(_test_single_i32_999, _single_val, 9, 3);
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


    //Actually this test can go out in the documentation because we are only dealing with u8 to i32 here
    // The struct is just one example of how this can be used at the higher level, there will be many more
    /*
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
    }
    */

    /*
    insert_value_at_position(&mut _test_single_i32, 333, 9, 3);
    assert_eq!(_test_single_i32, 1333000000);
    //println!("{:?}", _test_single_i32);
    //
    flush_value_to_zero(&mut _test_single_i32, 9, 3);
    assert_eq!(_test_single_i32, 1000000000);
    //println!("{:?}", _test_single_i32);
    //
    insert_value_at_position(&mut _test_single_i32, 333, 3, 3);
    assert_eq!(_test_single_i32, 1000000333);
    //println!("{:?}", _test_single_i32);
    //
    //
    insert_value_at_position(&mut _test_single_i32, 333, 6, 3);
    assert_eq!(_test_single_i32, 1000333333);
    //println!("{:?}", _test_single_i32);
    //
    insert_value_at_position(&mut _test_single_i32, 8, 9, 3);
    assert_eq!(_test_single_i32, 1008333333);
    //println!("{:?}", _test_single_i32);
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    */
}
