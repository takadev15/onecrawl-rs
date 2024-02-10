use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageTables {
    pub page_id: String,
    pub table_str: String,
}
