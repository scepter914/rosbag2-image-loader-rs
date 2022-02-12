use crate::interface::rosbag2_image_interface::Rosbag2Images;
use crate::rosbag2::message::TopicMessage;
use crate::rosbag2::topic::Topic;
use rusqlite::Connection;

/// Load rosbag2 and construct the vector of Rosbag2Images interface struct.
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

    // Get image topic
    let mut image_topics: Vec<Topic> = Vec::new();
    for topic in topic_rows {
        if topic.as_ref().unwrap().is_image_topic() {
            image_topics.push(topic.unwrap());
        }
    }

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
        let message_ref = message.as_ref().unwrap();
        let message_topic_id = message_ref.topic_id;

        // If Rosbag2Image is registered, add the image
        for rosbag2_images in &mut rosbag2_images_vector {
            if message_topic_id == rosbag2_images.get_topic_id() {
                rosbag2_images.add_images(message_ref);
            }
        }

        // If Rosbag2Image is not registered, register new Rosbag2Image.
        image_topics.retain(|image_topic| {
            let is_delete: bool = message_topic_id == image_topic.id;
            if is_delete {
                let new_image = message_ref.deserialize(&image_topic.topic_type).unwrap();
                let mut new_rosbag2_images =
                    Rosbag2Images::from_topic(&image_topic, new_image.width(), new_image.height());
                new_rosbag2_images.add_images(message_ref);
                rosbag2_images_vector.push(new_rosbag2_images);
            }
            !is_delete
        })
    }
    Ok(rosbag2_images_vector)
}
