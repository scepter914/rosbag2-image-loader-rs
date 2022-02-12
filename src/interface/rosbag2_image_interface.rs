use crate::interface::stamped_image::StampedImage;
use crate::rosbag2::message::TopicMessage;
use crate::rosbag2::topic::Topic;
use image::RgbImage;

/// Rosbag2 images interface
/// This interface is same as [simple_image_interface](https://github.com/scepter914/simple-image-interface-rs)
pub struct Rosbag2Images {
    topic_id: u16,
    topic_name: String,
    topic_type: String,
    width: u32,
    height: u32,
    now_frame_index: usize,
    images: Vec<StampedImage>,
}

impl Rosbag2Images {
    pub fn new(
        topic_id_: u16,
        topic_name_: impl Into<String>,
        topic_type_: impl Into<String>,
        width_: u32,
        height_: u32,
    ) -> Self {
        Rosbag2Images {
            topic_id: topic_id_,
            topic_name: topic_name_.into(),
            topic_type: topic_type_.into(),
            width: width_,
            height: height_,
            now_frame_index: 0,
            images: Vec::new(),
        }
    }

    /// Init frame Topic
    pub fn from_topic(topic: &Topic, width_: u32, height_: u32) -> Self {
        Rosbag2Images::new(topic.id, &topic.name, &topic.topic_type, width_, height_)
    }

    /// Get frame from interface
    /// If interface do not get a image, return None
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

    /// Get frame from 0-1 ratio
    /// If the number of frames is 100 and ratio is 0.4, return frame[40]
    pub fn get_frame_from_ratio(&mut self, ratio: f32) -> Option<&image::RgbImage> {
        let image_size: f32 = self.images.len() as f32;
        let ratio_: f32 = f32::min(f32::max(0.0, ratio), 1.0);
        let frame_index: usize = (image_size * ratio_) as usize;
        self.now_frame_index = frame_index - 1;
        let output: Option<&image::RgbImage> = Some(self.images[self.now_frame_index].get_image());
        output
    }

    pub fn get_topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn get_topic_id(&self) -> u16 {
        self.topic_id
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn reset_frame_index(&mut self) {
        self.now_frame_index = 0;
    }

    /// Add image from topic message
    pub(crate) fn add_images(&mut self, message: &TopicMessage) {
        let image: RgbImage = message.deserialize(&self.topic_type).unwrap();
        self.images
            .push(StampedImage::new(message.timestamp, image))
    }
}
