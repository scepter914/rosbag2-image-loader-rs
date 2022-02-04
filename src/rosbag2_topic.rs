#[derive(Debug)]
pub struct Topic {
    pub id: u16,
    pub name: String,
    pub topic_type: String,
    pub serialization_format: String,
    pub offered_qos_profiles: String,
}

pub struct ImageTopicWithCameraInfo {
    pub image_topic_id: u16,
    pub camera_info_topic_id: u16,
}
