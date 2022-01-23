use rosbag2_image_loader::loader::load_images_from_rosbag2;

fn main() {
    let file_name: String =
        "data/rosbag/rosbag2_2022_01_09-13_49_29/rosbag2_2022_01_09-13_49_29_0.db3".to_string();
    let result = load_images_from_rosbag2(file_name).unwrap();
}
