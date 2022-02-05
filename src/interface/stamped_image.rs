use image::RgbImage;

pub struct StampedImage {
    timestamp: u64,
    image: RgbImage,
}

impl StampedImage {
    pub fn new(width: u32, height: u32, timestamp_: u64, data: Vec<u8>) -> Self {
        StampedImage {
            timestamp: timestamp_,
            image: RgbImage::from_vec(width, height, data).unwrap(),
        }
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_image(&self) -> &RgbImage {
        &self.image
    }
}
