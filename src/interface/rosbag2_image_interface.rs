use crate::interface::stamped_image::StampedImage;
use crate::rosbag2::message::TopicMessage;

pub struct Rosbag2Images {
    topic_id: u16,
    topic_name: String,
    width: u32,
    height: u32,
    now_frame_index: usize,
    images: Vec<StampedImage>,
}

impl Rosbag2Images {
    pub fn new(topic_id_: u16, topic_name_: impl Into<String>, width_: u32, height_: u32) -> Self {
        Rosbag2Images {
            topic_id: topic_id_,
            topic_name: topic_name_.into(),
            width: width_,
            height: height_,
            now_frame_index: 0,
            images: Vec::new(),
        }
    }

    pub fn get_frame(&mut self) -> Option<&image::RgbImage> {
        let output: Option<&image::RgbImage>;
        if self.now_frame_index > self.images.len() - 1 {
            output = None;
        } else {
            output = Some(self.images[self.now_frame_index].get_image());
            self.now_frame_index += 1;
        }
        output
    }

    pub fn get_topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn get_topic_id(&self) -> u16 {
        self.topic_id
    }

    pub fn reset_frame_index(&mut self) {
        self.now_frame_index = 0;
    }

    pub fn add_images(&mut self, message: &TopicMessage) {
        let topic_image_data: Vec<u8> = message.convert_message_to_image_vec();
        self.images.push(StampedImage::new(
            self.width,
            self.height,
            message.timestamp,
            topic_image_data,
        ))
    }

    pub fn get_frame_from_ratio(&mut self, ratio: f32) -> Option<&image::RgbImage> {
        let image_size: f32 = self.images.len() as f32;
        let ratio_: f32 = f32::min(f32::max(0.0, ratio), 1.0);
        let frame_index: usize = (image_size * ratio_) as usize;
        self.now_frame_index = frame_index - 1;
        let output: Option<&image::RgbImage> = Some(self.images[self.now_frame_index].get_image());
        output
    }
}
