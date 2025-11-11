use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub position: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,      
    pub message: String,      
    pub data: Option<T>,      
}
