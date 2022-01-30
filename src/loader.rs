use crate::rosbag2_image_interface::Rosbag2Images;
use crate::rosbag2_message::TopicMessage;
use rusqlite::Connection;

pub fn load_images_from_rosbag2(file_name: String) -> rusqlite::Result<Vec<Rosbag2Images>> {
    // DB connection
    let db_connection = Connection::open(file_name).unwrap();

    // Load topic definition
    let mut topics_definition = db_connection
        .prepare("SELECT id, name, type, serialization_format, offered_qos_profiles FROM topics")?;

    // Load topic messages
    let mut messages =
        db_connection.prepare("SELECT id, topic_id, timestamp, data FROM messages")?;
    let messages_iter = messages.query_map([], |row| {
        Ok(TopicMessage {
            message_id: row.get(0)?,
            topic_id: row.get(1)?,
            timestamp: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    // Make vector of rosbag2_images interface from topic definition
    let mut rosbag2_images_vector: Vec<Rosbag2Images> = Vec::new();

    // Convert to Rosbag2Images struct
    rosbag2_images_vector.push(Rosbag2Images::new(3, "hoge".to_string(), 640, 480));

    // Convert messages to rosbag2_images interface
    for message in messages_iter {
        let message_topic_id = message.as_ref().unwrap().topic_id;
        for rosbag2_images in &mut rosbag2_images_vector {
            //if topic_id == 3 {
            if message_topic_id == rosbag2_images.get_topic_id() {
                let message_timestamp = message.as_ref().unwrap().timestamp;
                let topic_image_data: Vec<u8> =
                    message.as_ref().unwrap().convert_message_to_image_vec();
                rosbag2_images.add_images(message_timestamp, topic_image_data);
            }
        }
    }
    Ok(rosbag2_images_vector)
}
