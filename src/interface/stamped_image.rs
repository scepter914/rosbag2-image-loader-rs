use image::RgbImage;

/// Image with timestamp.
pub struct StampedImage {
    timestamp: u64,
    image: RgbImage,
}

impl StampedImage {
    pub fn new(timestamp_: u64, image_: RgbImage) -> Self {
        StampedImage {
            timestamp: timestamp_,
            image: image_,
        }
    }

    #[allow(dead_code)]
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_image(&self) -> &RgbImage {
        &self.image
    }
}
