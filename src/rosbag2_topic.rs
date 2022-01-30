#[derive(Debug)]
pub struct TopicData {
    pub message_id: u64,
    pub topic_id: u16,
    pub timestamp: u64,
    pub data: Option<Vec<u8>>,
}

impl TopicData {
    /// Convert from ROS2 topic message to image vector.
    /// ROS2 image message has
    /// - std_msgs/Header header
    /// - uint32 height
    /// - uint32 width
    /// - string encoding
    /// - uint8 is_bigendian
    /// - uint32 step
    /// - uint8[] data
    ///
    /// message.data[0..51] have std_msgs/Header header ~ uint32 step
    pub fn convert_message_to_image_vec(&self) -> Vec<u8> {
        let image_topic_data: Vec<u8> = self.data.as_ref().unwrap().to_vec();
        image_topic_data[52..].to_vec()
    }
}
