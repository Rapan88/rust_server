use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DOCTYPE {
    NEWS,
    INSTALLS,
    DOCS,
    UPDATES,
}