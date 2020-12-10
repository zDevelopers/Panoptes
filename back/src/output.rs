use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub uuid: Uuid
}
