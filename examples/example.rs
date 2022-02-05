use image::{GrayImage, Luma, RgbImage};
use rosbag2_image_loader::interface::rosbag2_image_interface::Rosbag2Images;
use rosbag2_image_loader::loader::load_images_from_rosbag2;

fn my_image_proc(rgb_image: &RgbImage, frame_index: usize) {
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut gray_image = GrayImage::new(width, height);

    // for example gray scale
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            let gray_pixel = [((pixel[0] as f32 * 0.2126) as u32
                + (pixel[1] as f32 * 0.7152) as u32
                + (pixel[2] as f32 * 0.0722) as u32) as u8; 1];
            gray_image.put_pixel(i, j, Luma(gray_pixel));
        }
    }

    if frame_index % 100 == 0 {
        println!("save gray scale image {}", frame_index);
        rgb_image
            .save(format!("./data/result/raw_{}.png", frame_index))
            .unwrap();
        gray_image
            .save(format!("./data/result/gray_{}.png", frame_index))
            .unwrap();
    }
}

fn main() {
    let file_name: String =
        "data/rosbag/rosbag2_2022_01_09-13_49_29/rosbag2_2022_01_09-13_49_29_0.db3".to_string();
    // let file_name: String =
    //     "data/rosbag/rosbag2_2022_02_05-00_54_33/rosbag2_2022_02_05-00_54_33_0.db3".to_string();

    let mut interfaces: Vec<Rosbag2Images> = load_images_from_rosbag2(file_name).unwrap();
    let mut frame_index = 0;
    loop {
        frame_index += 1;
        let input_image = interfaces[0].get_frame();
        if input_image.is_none() {
            break;
        }
        my_image_proc(&input_image.unwrap(), frame_index);
    }
}
