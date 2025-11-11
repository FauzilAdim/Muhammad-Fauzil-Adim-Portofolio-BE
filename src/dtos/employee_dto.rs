use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeDTO {
    pub name: String,
    pub position: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeDTO {
    pub name: Option<String>,
    pub position: Option<String>,
    pub email: Option<String>,
}
