use crate::rosbag2_image::Rosbag2Images;
use image::RgbImage;
use rusqlite::Connection;

#[derive(Debug)]
pub struct TopicData {
    message_id: u64,
    topic_id: u16,
    timestamp: u64,
    data: Option<Vec<u8>>,
}

pub fn load_images_from_rosbag2(file_name: String) -> rusqlite::Result<Vec<Rosbag2Images>> {
    // DB connection
    let db_connection = Connection::open(file_name).unwrap();

    // Make vector of rosbag2_images interface from topic definition
    // let mut rosbag2_images_vector: Vec<Rosbag2Images> = Vec::new();
    let mut rosbag2_images_vector: Vec<Rosbag2Images> =
        vec![Rosbag2Images::new(3, "hoge".to_string(), 640, 480)];
    // Convert to Rosbag2Images struct

    // Load messages
    let mut messages =
        db_connection.prepare("SELECT id, topic_id, timestamp, data FROM messages")?;
    let messages_iter = messages.query_map([], |row| {
        Ok(TopicData {
            message_id: row.get(0)?,
            topic_id: row.get(1)?,
            timestamp: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    // Convert messages to rosbag2_images interface
    for message in messages_iter {
        let message_topic_id = message.as_ref().unwrap().topic_id;
        let message_timestamp = message.as_ref().unwrap().timestamp;
        for rosbag2_images in &mut rosbag2_images_vector {
            //if topic_id == 3 {
            if message_topic_id == rosbag2_images.get_topic_id() {
                let topic_image_data: Vec<u8> = convert_message_to_image_vec(&message);
                rosbag2_images.add_images(message_timestamp, topic_image_data);
            }
        }

        // println!("Image {}", message.unwrap().timestamp,);
        //let image = RgbImage::from_vec(640, 480, image_data[..921600].to_vec()).unwrap();
        // let save_file = format!("data/result/{}.png", message.as_ref().unwrap().timestamp);
        // image.save(save_file).unwrap();
    }
    Ok(rosbag2_images_vector)
}

/// Convert from ROS2 topic message to image vector.
/// ROS2 image message has
/// - std_msgs/Header header
/// - uint32 height
/// - uint32 width
/// - string encoding
/// - uint8 is_bigendian
/// - uint32 step
/// - uint8[] data
///
/// message.data[0..51] have std_msgs/Header header ~ uint32 step
fn convert_message_to_image_vec(
    message: &std::result::Result<TopicData, rusqlite::Error>,
) -> Vec<u8> {
    let image_topic_data: Vec<u8> = message.as_ref().unwrap().data.as_ref().unwrap().to_vec();
    image_topic_data[52..].to_vec()
}
