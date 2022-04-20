use serde::{Deserialize, Serialize};

use mongodb::bson::{oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub discord_id: u64,
    pub todos: Vec<String>,
}
