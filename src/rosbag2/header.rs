/// Get the std_msgs/msg/Header information.
///
/// Header has data as below
///
/// | the index of topic data | topic data      |
/// | ----------------------- | --------------- |
/// |                         | uint32 seq      |
/// |                         | time stamp      |
/// | 12 .. header_data_size  | string frame_id |
pub struct TopicHeaderInfo {
    pub frame_id: String,
    pub size: usize,
}

impl TopicHeaderInfo {}
