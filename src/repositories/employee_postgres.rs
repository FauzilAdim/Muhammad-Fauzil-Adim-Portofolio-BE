use crate::models::employee::Employee;
use crate::dtos::employee_dto::{CreateEmployeeDTO, UpdateEmployeeDTO};
use deadpool_postgres::Pool;
use uuid::Uuid;
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

pub struct EmployeePostgresRepo {
    pub pool: Pool,
}

fn from_row(row: Row) -> Employee {
    Employee {
        id: row.get::<_, Uuid>("id"),
        name: row.get("name"),
        position: row.get("position"),
        email: row.get("email"),
    }
}
 
impl EmployeePostgresRepo {
    pub async fn add(&self, dto: CreateEmployeeDTO) -> Result<Employee, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let id = Uuid::new_v4();
        let stmt = client.prepare("INSERT INTO employees (id, name, position, email) VALUES ($1, $2, $3, $4) RETURNING id, name, position, email").await.map_err(|e| e.to_string())?;
        let row = client.query_one(&stmt, &[&id as &(dyn ToSql + Sync), &dto.name, &dto.position, &dto.email]).await.map_err(|e| e.to_string())?;
        Ok(from_row(row))
    }

    pub async fn get_all(&self) -> Result<Vec<Employee>, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare("SELECT id, name, position, email FROM employees").await.map_err(|e| e.to_string())?;
        let rows = client.query(&stmt, &[]).await.map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(from_row).collect())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Employee, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare("SELECT id, name, position, email FROM employees WHERE id = $1").await.map_err(|e| e.to_string())?;
        let row = client.query_one(&stmt, &[&id as &(dyn ToSql + Sync)]).await.map_err(|e| e.to_string())?;
        Ok(from_row(row))
    }

    pub async fn update(&self, id: Uuid, dto: UpdateEmployeeDTO) -> Result<Employee, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let current = self.get_by_id(id).await?;
        let name = dto.name.unwrap_or(current.name);
        let position = dto.position.unwrap_or(current.position);
        let email = dto.email.unwrap_or(current.email);

        let stmt = client.prepare("UPDATE employees SET name = $1, position = $2, email = $3 WHERE id = $4 RETURNING id, name, position, email").await.map_err(|e| e.to_string())?;
        let row = client.query_one(&stmt, &[&name, &position, &email, &id as &(dyn ToSql + Sync)]).await.map_err(|e| e.to_string())?;
        Ok(from_row(row))
    }

    pub async fn delete(&self, id: Uuid) -> Result<u64, String> {
        let client = self.pool.get().await.map_err(|e| e.to_string())?;
        let stmt = client.prepare("DELETE FROM employees WHERE id = $1").await.map_err(|e| e.to_string())?;
        let res = client.execute(&stmt, &[&id as &(dyn ToSql + Sync)]).await.map_err(|e| e.to_string())?;
        Ok(res)
    }
}
