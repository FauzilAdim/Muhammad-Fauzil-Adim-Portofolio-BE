mod config;
mod dtos;
mod models;
mod repositories;
mod services;
mod handlers;

use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use actix_files as fs;
use services::employee_service::EmployeeService;
use services::project_service::ProjectService;
use repositories::employee_postgres::EmployeePostgresRepo;
use repositories::project_postgres::ProjectPostgresRepo;
use handlers::employee_handler::*;
use handlers::project_handler::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Setup PostgreSQL connection pool
    let pg_pool = config::get_pg_pool();
    
    // Employee service
    let employee_repo = EmployeePostgresRepo { pool: pg_pool.clone() };
    let employee_svc = web::Data::new(EmployeeService { pg_repo: employee_repo });
    
    // Project service
    let project_repo = ProjectPostgresRepo { pool: pg_pool.clone() };
    let project_svc = web::Data::new(ProjectService { pg_repo: project_repo });

    // Create uploads directory if not exists
    std::fs::create_dir_all("./uploads").unwrap_or_default();

    println!("🚀 Server starting on http://127.0.0.1:8080");
    println!("📊 Database: PostgreSQL");
    println!("🌐 CORS enabled for frontend");
    println!("📁 Static files served from /uploads");

    HttpServer::new(move || {
        // Konfigurasi CORS - Support both local and production
        let cors = Cors::default()
            // Local development
            .allowed_origin("http://localhost:3001")
            .allowed_origin("http://localhost:3002")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            // Production - Allow all Vercel domains
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".vercel.app") ||
                origin.as_bytes().ends_with(b"vercel.app")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                "accept",
                "authorization", 
                "content-type",
                "user-agent",
                "x-requested-with"
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(employee_svc.clone())
            .app_data(project_svc.clone())
            // Employee endpoints
            .service(add_employee)
            .service(get_all_employees)
            .service(get_employee_by_id)
            .service(update_employee)
            .service(delete_employee)
            // Project endpoints
            .service(add_project)
            .service(upload_project_image)
            .service(create_project_with_upload)
            .service(get_all_projects)
            .service(get_project_by_id)
            .service(update_project)
            .service(delete_project)
            // Serve static files (uploaded images)
            .service(fs::Files::new("/uploads", "./uploads").show_files_listing())
    })
    .bind((
        "0.0.0.0", 
        std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number")
    ))?
    .run()
    .await
}
