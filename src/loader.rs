use crate::interface::rosbag2_image_interface::Rosbag2Images;
use crate::rosbag2::image_topic::ImageTopicInfo;
use crate::rosbag2::image_topic::ImageTopicManager;
use crate::rosbag2::message::TopicMessage;
use crate::rosbag2::topic::Topic;
use rusqlite::Connection;

pub fn load_images_from_rosbag2(file_name: String) -> rusqlite::Result<Vec<Rosbag2Images>> {
    // DB connection
    let db_connection = Connection::open(file_name).unwrap();

    // Load topic definition
    let mut topics = db_connection
        .prepare("SELECT id, name, type, serialization_format, offered_qos_profiles FROM topics")?;
    let topic_rows = topics.query_map([], |row| {
        Ok(Topic {
            id: row.get(0)?,
            name: row.get(1)?,
            topic_type: row.get(2)?,
            serialization_format: row.get(3)?,
            offered_qos_profiles: row.get(4)?,
        })
    })?;

    // Buffer Topic to construct Rosbag2Image struct

    let mut image_topic_manager = ImageTopicManager::new();
    for topic in topic_rows {
        image_topic_manager.add_image_topic(topic.unwrap());
    }
    let mut image_topic_infos: Vec<ImageTopicInfo> = image_topic_manager.get_image_topic_infos();

    // Load topic messages
    let mut messages =
        db_connection.prepare("SELECT id, topic_id, timestamp, data FROM messages")?;
    let message_rows = messages.query_map([], |row| {
        Ok(TopicMessage {
            message_id: row.get(0)?,
            topic_id: row.get(1)?,
            timestamp: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    let mut rosbag2_images_vector: Vec<Rosbag2Images> = Vec::new();

    // Convert messages to rosbag2_images interface
    for message in message_rows {
        let message_topic_id = message.as_ref().unwrap().topic_id;
        // If Rosbag2Image is registered, add the image
        for rosbag2_images in &mut rosbag2_images_vector {
            if message_topic_id == rosbag2_images.get_topic_id() {
                rosbag2_images.add_images(message.as_ref().unwrap());
            }
        }
        image_topic_infos.retain(|image_topic_info| {
            let is_delete: bool = message_topic_id == image_topic_info.camera_info_topic_id;
            if is_delete {
                let (width, height) = message.as_ref().unwrap().convert_message_to_camera_info();
                rosbag2_images_vector.push(Rosbag2Images::new(
                    image_topic_info.image_topic_id,
                    &image_topic_info.image_topic_name,
                    width,
                    height,
                ));
            }
            !is_delete
        })

        // If Rosbag2Image is not registered, register new Rosbag2Image.
        // rosbag2_images_vector.push(Rosbag2Images::new(3, "hoge".to_string(), 640, 480));
    }
    Ok(rosbag2_images_vector)
}
