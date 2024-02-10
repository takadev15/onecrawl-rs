use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageStyle {
    pub page_id: String,
    pub style: String,
}
