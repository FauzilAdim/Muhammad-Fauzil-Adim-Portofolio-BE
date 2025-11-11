use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::services::employee_service::EmployeeService;
use crate::dtos::employee_dto::{CreateEmployeeDTO, UpdateEmployeeDTO};
use crate::models::employee::Employee;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

#[post("/api/employees")]
pub async fn add_employee(
    svc: web::Data<EmployeeService>,
    body: web::Json<CreateEmployeeDTO>
) -> impl Responder {
    match svc.add(body.0).await {
        Ok(employee) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Employee added successfully".to_string(),
            data: Some(employee),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/api/employees")]
pub async fn get_all_employees(
    svc: web::Data<EmployeeService>
) -> impl Responder {
    match svc.get_all().await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: format!("{} employees found", list.len()),
            data: Some(list),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<Employee>> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/api/employees/{id}")]
pub async fn get_employee_by_id(
    svc: web::Data<EmployeeService>,
    id: web::Path<Uuid>
) -> impl Responder {
    match svc.get_by_id(id.into_inner()).await {
        Ok(employee) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Employee found".to_string(),
            data: Some(employee),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[put("/api/employees/{id}")]
pub async fn update_employee(
    svc: web::Data<EmployeeService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateEmployeeDTO>
) -> impl Responder {
    match svc.update(id.into_inner(), body.0).await {
        Ok(employee) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Employee updated successfully".to_string(),
            data: Some(employee),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[delete("/api/employees/{id}")]
pub async fn delete_employee(
    svc: web::Data<EmployeeService>,
    id: web::Path<Uuid>
) -> impl Responder {
    match svc.delete(id.into_inner()).await {
        Ok(deleted_count) if deleted_count > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "success".to_string(),
            message: "Employee deleted successfully".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Employee not found".to_string(),
            data: None,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}
