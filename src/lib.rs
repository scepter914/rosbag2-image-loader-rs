use image::RgbImage;
use rusqlite::Connection;

#[derive(Debug)]
struct Topic {
    id: i32,
    topic_id: i32,
    timestamp: i64,
    data: Option<Vec<u8>>,
}

struct Image {
    timestamp: i64,
    data: Option<Vec<u8>>,
}

pub fn load_rosbag2(file_name: String) -> rusqlite::Result<()> {
    let db_connection = Connection::open(file_name).unwrap();
    let mut messages =
        db_connection.prepare("SELECT id, topic_id, timestamp, data FROM messages")?;
    let messages_iter = messages.query_map([], |row| {
        Ok(Topic {
            id: row.get(0)?,
            topic_id: row.get(1)?,
            timestamp: row.get(2)?,
            data: row.get(3)?,
        })
    })?;
    for message in messages_iter {
        if message.as_ref().unwrap().topic_id == 3 {
            // println!("Image {}", message.unwrap().timestamp,);
            let image_data: Vec<u8> = message.as_ref().unwrap().data.as_ref().unwrap().to_vec();
            //println!("Image {}", image_data.len());
            let image = RgbImage::from_vec(640, 480, image_data[52..].to_vec()).unwrap();
            //let image = RgbImage::from_vec(640, 480, image_data[..921600].to_vec()).unwrap();
            let save_file = format!("data/result/{}.png", message.as_ref().unwrap().timestamp);
            image.save(save_file).unwrap();
        }
        if message.as_ref().unwrap().topic_id == 4 {
            println!("Camera Info {:?}", message.unwrap());
        }
    }
    Ok(())
}
