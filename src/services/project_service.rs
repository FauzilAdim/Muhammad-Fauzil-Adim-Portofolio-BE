use crate::dtos::project_dto::{CreateProjectDTO, UpdateProjectDTO};
use crate::models::project::Project;
use crate::repositories::project_postgres::ProjectPostgresRepo;
use uuid::Uuid;

pub struct ProjectService {
    pub pg_repo: ProjectPostgresRepo,
}

impl ProjectService {
    pub async fn add(&self, dto: CreateProjectDTO) -> Result<Project, String> {
        self.pg_repo.add(dto).await
    }
    
    pub async fn get_all(&self) -> Result<Vec<Project>, String> {
        self.pg_repo.get_all().await
    }
    
    pub async fn get_by_category(&self, category: String) -> Result<Vec<Project>, String> {
        self.pg_repo.get_by_category(category).await
    }
    
    pub async fn get_by_id(&self, id: Uuid) -> Result<Project, String> {
        self.pg_repo.get_by_id(id).await
    }
    
    pub async fn update(&self, id: Uuid, dto: UpdateProjectDTO) -> Result<Project, String> {
        self.pg_repo.update(id, dto).await
    }
    
    pub async fn delete(&self, id: Uuid) -> Result<u64, String> {
        self.pg_repo.delete(id).await
    }
}
