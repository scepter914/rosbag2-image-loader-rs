use crate::rosbag2::topic::Topic;

#[derive(Debug)]
pub struct ImageTopicInfo {
    pub image_topic_name: String,
    pub image_topic_id: u16,
    pub camera_info_topic_id: u16,
}

impl ImageTopicInfo {
    fn new(
        image_topic_name_: impl Into<String>,
        image_topic_id_: u16,
        camera_info_topic_id_: u16,
    ) -> Self {
        ImageTopicInfo {
            image_topic_name: image_topic_name_.into(),
            image_topic_id: image_topic_id_,
            camera_info_topic_id: camera_info_topic_id_,
        }
    }
}
pub struct ImageTopicManager {
    image_topics: Vec<Topic>,
    camera_info_topics: Vec<Topic>,
}

impl ImageTopicManager {
    pub fn new() -> Self {
        ImageTopicManager {
            image_topics: Vec::new(),
            camera_info_topics: Vec::new(),
        }
    }

    pub fn add_image_topic(&mut self, topic: Topic) {
        if &topic.topic_type == "sensor_msgs/msg/Image" {
            self.image_topics.push(topic);
        } else if &topic.topic_type == "sensor_msgs/msg/CameraInfo" {
            self.camera_info_topics.push(topic);
        }
    }

    /// Get vector of ImageTopicId
    pub fn get_image_topic_infos(&self) -> Vec<ImageTopicInfo> {
        let mut image_topic_infos: Vec<ImageTopicInfo> = Vec::new();
        for image_topic in &self.image_topics {
            for camera_info_topic in &self.camera_info_topics {
                if is_same_name_space(&image_topic.name, &camera_info_topic.name) {
                    image_topic_infos.push(ImageTopicInfo::new(
                        &image_topic.name,
                        image_topic.id,
                        camera_info_topic.id,
                    ));
                }
            }
        }
        image_topic_infos
    }
}

fn is_same_name_space(image_topic_name: &str, camera_info_topic_name: &str) -> bool {
    // root topic from image topic
    let mut root_from_image_topic: Option<String> =
        get_root_topic_name(image_topic_name, "/image_raw/compressed_image");
    if root_from_image_topic.is_none() {
        root_from_image_topic = get_root_topic_name(camera_info_topic_name, "/image_raw");
    }

    // root topic from camera info topic
    let root_from_camera_info_topic: Option<String> =
        get_root_topic_name(image_topic_name, "/camera_info");

    let output: bool = root_from_image_topic == root_from_camera_info_topic;
    output
}

fn get_root_topic_name(topic_name: &str, keyword: &str) -> Option<String> {
    let common_topic_name: Option<String>;
    if topic_name.contains(keyword) {
        let last_index: usize = topic_name.chars().count() - keyword.chars().count();
        let topic_name_vec: Vec<char> = topic_name.chars().collect();
        common_topic_name = Some(topic_name_vec[..last_index].iter().collect());
    } else {
        common_topic_name = None;
    }
    common_topic_name
}
