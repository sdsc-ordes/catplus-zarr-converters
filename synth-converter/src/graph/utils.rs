use uuid::Uuid;

pub fn generate_unique_identifier() -> String {
    Uuid::new_v4().to_string()
}
