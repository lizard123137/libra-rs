use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct BookEntity {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub summary: String,
}