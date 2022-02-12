use crate::rosbag2::message_function::{get_header, get_string, get_u32};
use image::RgbImage;

use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub struct TopicMessage {
    pub message_id: u64,
    pub topic_id: u16,
    pub timestamp: u64,
    pub data: Option<Vec<u8>>,
}

impl TopicMessage {
    pub fn deserialize(&self, topic_type: impl Into<String>) -> Option<RgbImage> {
        match topic_type.into().as_str() {
            "sensor_msgs/msg/Image" => Some(self.deserialize_image_message()),
            //"sensor_msgs/msg/CompressedImage" => Some(self.deserialize_compressed_image_message()),
            _ => None,
        }
    }

    /// Deserialize sensor_msgs/msg/Image to image vector.
    /// ROS2 image message has
    /// | the index of topic data              | topic data             |
    /// | ------------------------------------ | ---------------------- |
    /// | 0..(header.size - 1)                 | std_msgs/Header header |
    /// | header.size..(header.size + 3)       | uint32 height          |
    /// | (header.size + 4)..(header.size + 7) | uint32 width           |
    /// |                                      | string encoding        |
    /// |                                      | uint8 is_bigendian     |
    /// |                                      | uint32 step            |
    /// | (header.datasize + 28)               | uint8[] data           |
    pub fn deserialize_image_message(&self) -> RgbImage {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header = get_header(&topic_data);
        let height = get_u32(&topic_data, header.size);
        let width = get_u32(&topic_data, header.size + 4);
        RgbImage::from_vec(width, height, topic_data[(header.size + 28)..].to_vec()).unwrap()
    }

    /*
    /// Deserialize sensor_msgs/msg/CompressedImage to image vector.
    /// ROS2 image message has
    /// | the index of topic data               | topic data             |
    /// | ------------------------------------- | ---------------------- |
    /// | 0 .. (header_len - 1)                 | std_msgs/Header header |
    /// | header_len .. (string_last_index - 1) | string format          |
    /// | (string_last_index)..                 | uint8[] data           |
    pub fn deserialize_compressed_image_message(&self) -> RgbImage {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header = get_header(&topic_data);
        let (string, string_last_index) = get_string(&topic_data, header.size);

        let string_last_index = self.get_index_after_string(&topic_data, header_len - 1);
        let format_string =
            String::from_utf8(topic_data[(header_len + 3)..(string_last_index - 1)].to_vec());

        println!(
            "{}, {}, {:?}, {:?} \n {:?}",
            topic_data.len(),
            header_len,
            frame_id,
            format_string,
            &topic_data[0..40]
        );

        // let a: RgbImage = RgbImage::from_raw(topic_data[header_len..(header_len)]).into_rgb8();

        let mut file = File::create("data/result/hoge.png").unwrap();
        file.write_all(&topic_data[(header_len + 9)..]).unwrap();
        RgbImage::from_vec(110, 200, topic_data[(header_len + 9)..].to_vec()).unwrap()
    }
    */

    /// | the index of topic data              | topic data                       |
    /// | ------------------------------------ | -------------------------------- |
    /// | 0..(header.size - 1)                 | std_msgs/Header header           |
    /// | header.size..(header.size + 3)       | uint32 height                    |
    /// | (header.size + 4)..(header.size + 7) | uint32 width                     |
    /// |                                      | string distortion_model          |
    /// |                                      | float64[] D                      |
    /// |                                      | float64[9] K                     |
    /// |                                      | float64[9] R                     |
    /// |                                      | float64[12] P                    |
    /// |                                      | uint32 binning_x                 |
    /// |                                      | uint32 binning_y                 |
    /// |                                      | sensor_msgs/RegionOfInterest roi |
    pub fn convert_message_to_camera_info(&self) -> (u32, u32) {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header = get_header(&topic_data);
        let height = get_u32(&topic_data, header.size);
        let width = get_u32(&topic_data, header.size + 4);
        (width, height)
    }
}
