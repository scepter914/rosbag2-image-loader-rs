use image::RgbImage;

pub struct ImageMessage {
    timestamp: u64,
    image: RgbImage,
}

impl ImageMessage {
    pub fn new(width: u32, height: u32, timestamp_: u64, data: Vec<u8>) -> Self {
        ImageMessage {
            timestamp: timestamp_,
            image: RgbImage::from_vec(width, height, data).unwrap(),
        }
    }

    pub fn get_image(&self) -> RgbImage {
        self.image
    }
}
