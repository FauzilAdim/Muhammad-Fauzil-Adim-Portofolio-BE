use crate::models::project::Project;
use crate::dtos::project_dto::{CreateProjectDTO, UpdateProjectDTO};
use deadpool_postgres::Pool;
use uuid::Uuid;
use tokio_postgres::Row;
use tokio_postgres::types::{Type, ToSql};
use std::time::SystemTime;

pub struct ProjectPostgresRepo {
    pub pool: Pool,
}

fn from_row(row: Row) -> Project {
    // Get JSONB images as string and parse to Vec<String>
    let images_str: String = row.get("images");
    let images: Vec<String> = serde_json::from_str(&images_str)
        .unwrap_or_else(|_| vec![]);
    
    // Get timestamps as SystemTime and convert to string
    let created_at: Option<SystemTime> = row.get("created_at");
    let updated_at: Option<SystemTime> = row.get("updated_at");
    
    Project {
        id: row.get::<_, Uuid>("id"),
        name: row.get("name"),
        description: row.get("description"),
        images,
        category: row.get("category"),
        created_at: created_at.map(|_| chrono::Utc::now().format("%d/%m/%Y %H:%M").to_string()),
        updated_at: updated_at.map(|_| chrono::Utc::now().format("%d/%m/%Y %H:%M").to_string()),
    }
}

impl ProjectPostgresRepo {
    pub async fn add(&self, dto: CreateProjectDTO) -> Result<Project, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let id = Uuid::new_v4();
        
        // Convert Vec<String> to JSON string
        let images_json = serde_json::to_string(&dto.images)
            .map_err(|e| format!("Failed to serialize images: {}", e))?;
        
        let stmt = client
            .prepare_typed(
                "INSERT INTO projects (id, name, description, images, category) 
                 VALUES ($1, $2, $3, $4::jsonb, $5) 
                 RETURNING id, name, description, images::text, category, created_at, updated_at",
                &[Type::UUID, Type::VARCHAR, Type::TEXT, Type::TEXT, Type::VARCHAR]
            )
            .await
            .map_err(|e| e.to_string())?;
        
        let row = client
            .query_one(
                &stmt, 
                &[
                    &id, 
                    &dto.name, 
                    &dto.description, 
                    &images_json,
                    &dto.category
                ]
            )
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(from_row(row))
    }

    pub async fn get_all(&self) -> Result<Vec<Project>, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client
            .prepare("SELECT id, name, description, images::text, category, created_at, updated_at FROM projects ORDER BY created_at DESC")
            .await
            .map_err(|e| e.to_string())?;
        let rows = client.query(&stmt, &[]).await.map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(from_row).collect())
    }

    pub async fn get_by_category(&self, category: String) -> Result<Vec<Project>, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client
            .prepare("SELECT id, name, description, images::text, category, created_at, updated_at FROM projects WHERE category = $1 ORDER BY created_at DESC")
            .await
            .map_err(|e| e.to_string())?;
        let rows = client.query(&stmt, &[&category]).await.map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(from_row).collect())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Project, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client
            .prepare("SELECT id, name, description, images::text, category, created_at, updated_at FROM projects WHERE id = $1")
            .await
            .map_err(|e| e.to_string())?;
        let row = client
            .query_one(&stmt, &[&id])
            .await
            .map_err(|e| e.to_string())?;
        Ok(from_row(row))
    }

    pub async fn update(&self, id: Uuid, dto: UpdateProjectDTO) -> Result<Project, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let current = self.get_by_id(id).await?;
        
        let name = dto.name.unwrap_or(current.name);
        let description = dto.description.unwrap_or(current.description);
        let images = dto.images.unwrap_or(current.images);
        let category = dto.category.unwrap_or(current.category);
        
        // Convert Vec<String> to JSON string
        let images_json = serde_json::to_string(&images)
            .map_err(|e| format!("Failed to serialize images: {}", e))?;

        let stmt = client
            .prepare_typed(
                "UPDATE projects SET name = $1, description = $2, images = $3::jsonb, category = $4, updated_at = CURRENT_TIMESTAMP 
                 WHERE id = $5 
                 RETURNING id, name, description, images::text, category, created_at, updated_at",
                &[Type::VARCHAR, Type::TEXT, Type::TEXT, Type::VARCHAR, Type::UUID]
            )
            .await
            .map_err(|e| e.to_string())?;
        
        let row = client
            .query_one(&stmt, &[&name, &description, &images_json, &category, &id])
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(from_row(row))
    }

    pub async fn delete(&self, id: Uuid) -> Result<u64, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client
            .prepare("DELETE FROM projects WHERE id = $1")
            .await
            .map_err(|e| e.to_string())?;
        let res = client
            .execute(&stmt, &[&id as &(dyn ToSql + Sync)])
            .await
            .map_err(|e| e.to_string())?;
        Ok(res)
    }
}
