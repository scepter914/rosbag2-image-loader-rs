use crate::image_message::ImageMessage;

pub struct Rosbag2Images {
    topic_id: u16,
    topic_name: String,
    now_frame_index: usize,
    images: Vec<ImageMessage>,
}

impl Rosbag2Images {
    pub fn new(topic_id_: u16, topic_name_: String) -> Self {
        Rosbag2Images {
            topic_id: topic_id_,
            topic_name: topic_name_,
            now_frame_index: 0,
            images: Vec::new(),
        }
    }

    pub fn get_frame(&self) -> Option<image::RgbImage> {
        let output: Option<image::RgbImage>;
        if self.now_frame_index > self.images.len() {
            output = None;
        } else {
            output = Some(self.images[self.now_frame_index].get_image());
            self.now_frame_index += 1;
        }
        output
    }

    pub fn reset_frame_index(&self) {
        self.now_frame_index = 0;
    }
}
