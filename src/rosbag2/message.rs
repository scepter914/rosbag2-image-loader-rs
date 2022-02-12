use crate::rosbag2::message_function::{get_header, get_u32};
use image::RgbImage;

pub struct TopicMessage {
    pub message_id: u64,
    pub topic_id: u16,
    pub timestamp: u64,
    pub data: Option<Vec<u8>>,
}

impl TopicMessage {
    /// Deserialize image message.
    pub fn deserialize(&self, topic_type: impl Into<String>) -> Option<RgbImage> {
        match topic_type.into().as_str() {
            "sensor_msgs/msg/Image" => Some(self.deserialize_image_message()),
            _ => None,
        }
    }

    /// Deserialize sensor_msgs/msg/Image to image vector.
    ///
    /// ROS2 image message has data as below.
    ///
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

    /// Convert CameraInfo to (width, height).
    ///
    /// CameraInfo has data as below.
    ///
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
    #[allow(dead_code)]
    pub fn convert_message_to_camera_info(&self) -> (u32, u32) {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header = get_header(&topic_data);
        let height = get_u32(&topic_data, header.size);
        let width = get_u32(&topic_data, header.size + 4);
        (width, height)
    }
}
