use image::RgbImage;
use std::convert::TryInto;

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
            "sensor_msgs/msg/CompressedImage" => Some(self.deserialize_compressed_image_message()),
            _ => None,
        }
    }

    /// Deserialize sensor_msgs/msg/Image to image vector.
    /// ROS2 image message has
    /// | the index of topic data              | topic data             |
    /// | ------------------------------------ | ---------------------- |
    /// | 0 .. (header_len - 1)                | std_msgs/Header header |
    /// | header_len .. (header_len + 3)       | uint32 height          |
    /// | (header_len + 4) .. (header_len + 7) | uint32 width           |
    /// |                                      | string encoding        |
    /// |                                      | uint8 is_bigendian     |
    /// |                                      | uint32 step            |
    /// | (header_len + 28)                    | uint8[] data           |
    pub fn deserialize_image_message(&self) -> RgbImage {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header_len: usize = self.get_header_len(&topic_data);

        let height_u8: [u8; 4] = slice_to_array(&topic_data[header_len..(header_len + 4)]);
        let height: u32 = unsafe { std::mem::transmute(height_u8) };

        let width_u8: [u8; 4] = slice_to_array(&topic_data[(header_len + 4)..(header_len + 8)]);
        let width: u32 = unsafe { std::mem::transmute(width_u8) };

        let string = String::from_utf8(topic_data[16..(header_len - 1)].to_vec());
        // println!("{}, {:?}, {:?}", header_len, string, &topic_data[10..40]);
        // println!("{}, {}", width, height);

        let image =
            RgbImage::from_vec(width, height, topic_data[(header_len + 28)..].to_vec()).unwrap();
        image
    }

    pub fn deserialize_compressed_image_message(&self) -> RgbImage {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let header_len: usize = self.get_header_len(&topic_data);

        let string = String::from_utf8(topic_data[16..(header_len - 1)].to_vec());
        println!("{}, {:?}, {:?}", header_len, string, &topic_data[10..50]);

        let index_data = self.get_index_after_string(&topic_data, header_len - 1);
        let format_string =
            String::from_utf8(topic_data[(header_len + 3)..(index_data - 1)].to_vec());
        println!("{:?}", format_string);
        // let a: RgbImage = RgbImage::from_raw(topic_data[header_len..(header_len)]).into_rgb8();

        RgbImage::from_vec(110, 200, topic_data[(header_len + 9)..].to_vec()).unwrap()
    }

    /// Get the length of std_msgs/msg/Header
    /// | the index of topic data | topic data      |
    /// | ----------------------- | --------------- |
    /// |                         | uint32 seq      |
    /// |                         | time stamp      |
    /// | 12 ..                   | string frame_id |
    fn get_header_len(&self, topic_data: &[u8]) -> usize {
        self.get_index_after_string(topic_data, 12)
    }

    fn get_index_after_string(&self, topic_data: &[u8], start_index: usize) -> usize {
        start_index + 4 + topic_data[start_index] as usize + 1
    }

    /// | the index of topic data | topic data |
    /// | -- | --  |
    /// | 0-19 |  std_msgs/Header header |
    /// | 20-23 | uint32 height |
    /// | 24-27 | uint32 width |
    /// |  | string distortion_model |
    /// |  | float64[] D |
    /// |  | float64[9] K |
    /// |  | float64[9] R |
    /// |  | float64[12] P |
    /// |  | uint32 binning_x |
    /// |  | uint32 binning_y |
    /// |  | sensor_msgs/RegionOfInterest roi |
    pub fn convert_message_to_camera_info(&self) -> (u32, u32) {
        let topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        let height_u8: [u8; 4] = slice_to_array(&topic_data[20..24]);
        let width_u8: [u8; 4] = slice_to_array(&topic_data[24..28]);
        let height: u32 = unsafe { std::mem::transmute(height_u8) };
        let width: u32 = unsafe { std::mem::transmute(width_u8) };
        (width, height)
    }
}

fn slice_to_array(slice: &[u8]) -> [u8; 4] {
    slice.try_into().expect("slice with incorrect length")
}
