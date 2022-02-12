use crate::rosbag2::header::TopicHeaderInfo;
use std::convert::TryInto;

/// Get header information.
pub fn get_header(topic_data: &[u8]) -> TopicHeaderInfo {
    let (frame_id_, header_data_size_) = get_string(topic_data, 12);
    TopicHeaderInfo {
        frame_id: frame_id_,
        size: header_data_size_,
    }
}

/// Get string information.
///
/// Return.
///
/// - String: string data
/// - usize: end_index
pub fn get_string(topic_data: &[u8], start_index: usize) -> (String, usize) {
    let end_index = start_index + 4 + topic_data[start_index] as usize + 1;
    let string =
        String::from_utf8(topic_data[(start_index + 4)..(end_index - 1)].to_vec()).unwrap();
    (string, end_index)
}

pub fn get_u32(topic_data: &[u8], index: usize) -> u32 {
    let u8_array = u32_slice_to_array(&topic_data[index..(index + 4)]);
    unsafe { std::mem::transmute(u8_array) }
}

pub fn u32_slice_to_array(slice: &[u8]) -> [u8; 4] {
    slice.try_into().expect("slice with incorrect length")
}
