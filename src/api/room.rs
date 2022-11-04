use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    msg: String,
    owner: i64,
}
