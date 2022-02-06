use std::convert::TryInto;

pub struct TopicMessage {
    pub message_id: u64,
    pub topic_id: u16,
    pub timestamp: u64,
    pub data: Option<Vec<u8>>,
}

impl TopicMessage {
    /// Convert from ROS2 topic message to image vector.
    /// ROS2 image message has
    /// | the index of topic data | topic data |
    /// | -- | --  |
    /// | 0-19 |  std_msgs/Header header |
    /// | 20-23 | uint32 height |
    /// | 24-27 | uint32 width |
    /// |  |  string encoding |
    /// |  |  uint8 is_bigendian |
    /// |  |  uint32 step |
    /// | 52- |  uint8[] data |
    pub fn convert_message_to_image_vec(&self) -> Vec<u8> {
        let image_topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        image_topic_data[52..].to_vec()
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
