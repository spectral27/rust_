use uuid::Uuid;

pub fn get_id() -> String {
    let uuid = Uuid::new_v4().to_string().replace("-", "");
    uuid[0..16].to_string()
}
