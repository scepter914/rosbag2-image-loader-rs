/// Topic struct
#[derive(Debug)]
pub struct Topic {
    pub id: u16,
    pub name: String,
    pub topic_type: String,
    pub serialization_format: String,
    pub offered_qos_profiles: String,
}

impl Topic {
    pub fn is_image_topic(&self) -> bool {
        matches!(
            self.topic_type.as_str(),
            "sensor_msgs/msg/Image" | "sensor_msgs/msg/CompressedImage"
        )
    }
}
