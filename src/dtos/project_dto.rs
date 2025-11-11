use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectDTO {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub category: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProjectDTO {
    pub name: Option<String>,
    pub description: Option<String>,
    pub images: Option<Vec<String>>,
    pub category: Option<String>,
}
