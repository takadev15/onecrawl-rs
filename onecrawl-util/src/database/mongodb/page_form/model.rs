use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageForm {
    pub page_id: String,
    pub form: String,
}
