#[derive(Debug)]
pub struct Topic {
    pub id: u16,
    pub name: String,
    pub topic_type: String,
    pub serialization_format: String,
    pub offered_qos_profiles: String,
}

pub struct ImageTopicId {
    pub image_topic_id: u16,
    pub camera_info_topic_id: u16,
}

/// Get vector of ImageTopicId
pub fn get_image_topic_ids(
    image_topics: Vec<Topic>,
    camera_info_topics: Vec<Topic>,
) -> Vec<ImageTopicId> {
    let mut rosbag2_images_vector: Vec<ImageTopicId> = Vec::new();
    for image_topic in image_topics {
        for camera_info_topic in &camera_info_topics {
            if is_same_name_space(&image_topic.name, &camera_info_topic.name) {}
        }
    }

    rosbag2_images_vector
}
