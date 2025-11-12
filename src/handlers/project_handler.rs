use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use uuid::Uuid;
use std::io::Write;
use crate::services::project_service::ProjectService;
use crate::dtos::project_dto::{CreateProjectDTO, UpdateProjectDTO};
use crate::models::project::Project;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

#[post("/api/projects")]
pub async fn add_project(
    svc: web::Data<ProjectService>,
    body: web::Json<CreateProjectDTO>
) -> impl Responder {
    // Validate that images array is not empty
    if body.images.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "At least one image is required".to_string(),
            data: None,
        });
    }
    
    match svc.add(body.0).await {
        Ok(project) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Project added successfully".to_string(),
            data: Some(project),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[post("/api/projects/upload")]
pub async fn upload_project_image(
    mut payload: Multipart,
) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Error reading multipart: {}", e),
                    data: None,
                });
            }
        };

        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map(|f| sanitize_filename::sanitize(f))
            .unwrap_or_else(|| format!("{}.png", Uuid::new_v4()));

        let filepath = format!("./uploads/{}", filename);

        // Create uploads directory if not exists
        std::fs::create_dir_all("./uploads").unwrap_or_default();

        let mut f = match web::block(move || std::fs::File::create(filepath.clone()))
            .await
        {
            Ok(Ok(file)) => file,
            Ok(Err(e)) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Error creating file: {}", e),
                    data: None,
                });
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Blocking error: {}", e),
                    data: None,
                });
            }
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Error reading chunk: {}", e),
                        data: None,
                    });
                }
            };
            
            if let Err(e) = f.write_all(&data) {
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Error writing file: {}", e),
                    data: None,
                });
            }
        }

        return HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Image uploaded successfully".to_string(),
            data: Some(serde_json::json!({
                "filename": filename,
                "url": format!("/uploads/{}", filename)
            })),
        });
    }

    HttpResponse::BadRequest().json(ApiResponse::<()> {
        status: "error".to_string(),
        message: "No file uploaded".to_string(),
        data: None,
    })
}

#[post("/api/projects/create-with-upload")]
pub async fn create_project_with_upload(
    svc: web::Data<ProjectService>,
    mut payload: Multipart,
) -> impl Responder {
    use crate::services::cloudinary::CloudinaryService;
    
    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut category: Option<String> = None;
    let mut image_urls: Vec<String> = Vec::new();
    
    let cloudinary = match CloudinaryService::new() {
        Ok(service) => service,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Cloudinary not configured: {}. Please set CLOUDINARY_CLOUD_NAME environment variable.", e),
                data: None,
            });
        }
    };

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Error reading multipart: {}", e),
                    data: None,
                });
            }
        };

        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "name" => {
                let mut bytes = web::BytesMut::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    bytes.extend_from_slice(&data);
                }
                name = Some(String::from_utf8(bytes.to_vec()).unwrap_or_default());
            }
            "description" => {
                let mut bytes = web::BytesMut::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    bytes.extend_from_slice(&data);
                }
                description = Some(String::from_utf8(bytes.to_vec()).unwrap_or_default());
            }
            "category" => {
                let mut bytes = web::BytesMut::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    bytes.extend_from_slice(&data);
                }
                category = Some(String::from_utf8(bytes.to_vec()).unwrap_or_default());
            }
            "file" | "image" | "files" | "images" => {
                let original_filename = content_disposition
                    .get_filename()
                    .map(|f| sanitize_filename::sanitize(f))
                    .unwrap_or_else(|| format!("{}.png", Uuid::new_v4()));
                
                let extension = original_filename
                    .rsplit('.')
                    .next()
                    .unwrap_or("png");
                
                let index = image_urls.len();
                let filename = format!("{}_{:03}_{}.{}", 
                    Uuid::new_v4(), 
                    index, 
                    chrono::Utc::now().timestamp_millis(),
                    extension
                );

                // Collect image data
                let mut image_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(data) => data,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                status: "error".to_string(),
                                message: format!("Error reading chunk: {}", e),
                                data: None,
                            });
                        }
                    };
                    image_data.extend_from_slice(&data);
                }

                // Upload to Cloudinary
                match cloudinary.upload_image(image_data, filename).await {
                    Ok(url) => {
                        image_urls.push(url);
                    }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: format!("Failed to upload image to Cloudinary: {}", e),
                            data: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    if name.is_none() || description.is_none() || category.is_none() || image_urls.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Missing required fields: name, description, category, and at least one file are required".to_string(),
            data: None,
        });
    }

    let dto = CreateProjectDTO {
        name: name.unwrap(),
        description: description.unwrap(),
        images: image_urls,
        category: category.unwrap(),
    };

    match svc.add(dto).await {
        Ok(project) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Project created successfully with uploaded images".to_string(),
            data: Some(project),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/api/projects")]
pub async fn get_all_projects(
    svc: web::Data<ProjectService>,
    query: web::Query<std::collections::HashMap<String, String>>
) -> impl Responder {
    let result = if let Some(category) = query.get("category") {
        svc.get_by_category(category.clone()).await
    } else {
        svc.get_all().await
    };

    match result {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: format!("{} projects found", list.len()),
            data: Some(list),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<Project>> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/api/projects/{id}")]
pub async fn get_project_by_id(
    svc: web::Data<ProjectService>,
    id: web::Path<Uuid>
) -> impl Responder {
    match svc.get_by_id(id.into_inner()).await {
        Ok(project) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Project found".to_string(),
            data: Some(project),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[put("/api/projects/{id}")]
pub async fn update_project(
    svc: web::Data<ProjectService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdateProjectDTO>
) -> impl Responder {
    match svc.update(id.into_inner(), body.0).await {
        Ok(project) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Project updated successfully".to_string(),
            data: Some(project),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[delete("/api/projects/{id}")]
pub async fn delete_project(
    svc: web::Data<ProjectService>,
    id: web::Path<Uuid>
) -> impl Responder {
    match svc.delete(id.into_inner()).await {
        Ok(deleted_count) if deleted_count > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "success".to_string(),
            message: "Project deleted successfully".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Project not found".to_string(),
            data: None,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}
