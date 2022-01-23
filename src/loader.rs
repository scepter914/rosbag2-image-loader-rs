use crate::rosbag2_image::Rosbag2Images;
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

    // Convert to Rosbag2Images struct
    let mut images: Vec<Rosbag2Images> = Vec::new();
    for message in messages_iter {
        let topic_id = message.as_ref().unwrap().topic_id;
        let timestamp = message.as_ref().unwrap().timestamp;
        if topic_id == 3 {
            let image_topic_data: Vec<u8> =
                message.as_ref().unwrap().data.as_ref().unwrap().to_vec();
            let image_data: Vec<u8> = image_topic_data[52..].to_vec();
            // let topic_image = convert_topic_data_to_image(640, 480, timestamp, image_data);

            // images.push(topic_image);

            // println!("Image {}", message.unwrap().timestamp,);
            //let image = RgbImage::from_vec(640, 480, image_data[..921600].to_vec()).unwrap();
            // let save_file = format!("data/result/{}.png", message.as_ref().unwrap().timestamp);
            // image.save(save_file).unwrap();
        }
    }
    Ok(images)
}

fn get_image_data() {}
