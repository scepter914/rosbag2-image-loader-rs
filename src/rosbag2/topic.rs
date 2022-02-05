#[derive(Debug)]
pub struct Topic {
    pub id: u16,
    pub name: String,
    pub topic_type: String,
    pub serialization_format: String,
    pub offered_qos_profiles: String,
}
