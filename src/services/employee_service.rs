use crate::dtos::employee_dto::{CreateEmployeeDTO, UpdateEmployeeDTO};
use crate::models::employee::Employee;
use crate::repositories::employee_postgres::EmployeePostgresRepo;
use uuid::Uuid;

pub struct EmployeeService {
    pub pg_repo: EmployeePostgresRepo,
}

impl EmployeeService {
    pub async fn add(&self, dto: CreateEmployeeDTO) -> Result<Employee, String> {
        self.pg_repo.add(dto).await
    }
    
    pub async fn get_all(&self) -> Result<Vec<Employee>, String> {
        self.pg_repo.get_all().await
    }
    
    pub async fn get_by_id(&self, id: Uuid) -> Result<Employee, String> {
        self.pg_repo.get_by_id(id).await
    }
    
    pub async fn update(&self, id: Uuid, dto: UpdateEmployeeDTO) -> Result<Employee, String> {
        self.pg_repo.update(id, dto).await
    }
    
    pub async fn delete(&self, id: Uuid) -> Result<u64, String> {
        self.pg_repo.delete(id).await
    }
}
