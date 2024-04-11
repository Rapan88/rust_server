use actix_multipart::Multipart;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, web};
use futures::{TryStreamExt};
use tokio::fs::{File, remove_file};
use tokio::io::{AsyncWriteExt};
use actix_web::http::StatusCode;
use mongodb::{Collection, Cursor};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use serde::Deserialize;
use uuid::Uuid;
use crate::enums::doc_type::DOCTYPE;
use crate::models::document::Document;

#[derive(Deserialize)]
pub struct QueryType {
    pub doc_type: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryId {
    pub id: String,
}


pub async fn check_health(req: HttpRequest) -> impl Responder {
    let ip = req.connection_info().peer_addr().unwrap_or_default().to_string();
    println!("IP клієнта: {}", ip);
    HttpResponse::Ok().body(ip)
}

pub async fn save_files(mut payload: Multipart, collection: web::Data<Collection<Document>>) -> Result<HttpResponse, Error> {
    let dir: &str = "./uploads/";
    let mut doc = Document::default();

    doc.id = String::from(Uuid::new_v4());

    while let Some(mut field) = payload.try_next().await? {
        if field.name().starts_with("file") {
            let destination: String = format!(
                "{}{}",
                dir,
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: File = File::create(&destination).await.unwrap();

            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            doc.file_names.push(format!("{}", field.content_disposition().get_filename().unwrap()));
        } else {
            let mut field_data = Vec::new();

            while let Ok(Some(chunk)) = field.try_next().await {
                field_data.extend_from_slice(&chunk);
            }

            let field_value = match String::from_utf8(field_data) {
                Ok(value) => value,
                Err(error) => {
                    return Err(actix_web::error::InternalError::new(
                        error,
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                        .into());
                }
            };

            match field.name() {
                "text" => {
                    doc.text = field_value;
                }
                "description" => {
                    doc.description = field_value;
                }
                "path" => {
                    doc.path = Option::from(field_value);
                }
                "doc_type" => {
                    let doc_type = match field_value.as_str() {
                        "NEWS" => DOCTYPE::NEWS,
                        "INSTALLS" => DOCTYPE::INSTALLS,
                        "DOCS" => DOCTYPE::DOCS,
                        "UPDATES" => DOCTYPE::UPDATES,
                        _ => {
                            println!("Невідомий тип документа: {}", field_value);
                            continue;
                        }
                    };
                    doc.doc_type = doc_type
                }
                _ => {}
            }
        }
    }

    collection.insert_one(&doc, None).await.expect("Bad save");
    let json_doc = serde_json::to_string(&doc).expect("Failed to serialize documents to JSON");
    Ok(HttpResponse::Ok().content_type("application/json").body(json_doc))
}

pub async fn get_docs(query_params: web::Query<QueryType>, collection: web::Data<Collection<Document>>) -> Result<HttpResponse, Error> {
    let mut cursor: Cursor<Document>;
    if let Some(q) = &query_params.doc_type {
        cursor = match collection.find(doc! { "doc_type": q }, None).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(Error::from(actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)))
        };
    } else {
        cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(Error::from(actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)))
        };
    }


    let mut docs: Vec<Document> = Vec::new();

    while let Some(doc) = cursor.try_next().await.expect("Some error message") {
        docs.push(doc)
    }

    let json_docs = serde_json::to_string(&docs).expect("Failed to serialize documents to JSON");

    Ok(HttpResponse::Ok().content_type("application/json").body(json_docs))
}

pub async fn get_latest_doc(query_params: web::Query<QueryType>, collection: web::Data<Collection<Document>>) -> Result<HttpResponse, Error> {
    let query = match &query_params.doc_type {
        Some(q) => doc! { "doc_type": q },
        None => doc! {},
    };

    let find_options = FindOptions::builder()
        .sort(doc! { "date": -1 }) // Сортуємо за спаданням дати
        .limit(1) // Беремо лише один документ
        .build();

    let mut cursor = match collection.find(query, find_options).await {
        Ok(cursor) => cursor,
        Err(e) => return Err(Error::from(actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)))
    };

    let mut docs: Vec<Document> = Vec::new();

    // Цикл відображення документів в вектор
    while let Some(doc) = cursor.try_next().await.expect("Some error message") {
        docs.push(doc)
    }

    // Конвертуємо вектор документів в JSON
    let json_docs = serde_json::to_string(&docs).expect("Failed to serialize documents to JSON");

    Ok(HttpResponse::Ok().content_type("application/json").body(json_docs))
}

pub async fn delete_doc(query_id: web::Query<QueryId>, collection: web::Data<Collection<Document>>) -> Result<HttpResponse, Error> {
    let filter = doc! {"id": &query_id.id};
    let doc: Document = match collection.find_one(filter.clone(), None).await {
        Ok(doc) => doc.unwrap(),
        Err(e) => return Err(Error::from(actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)))
    };

    for doc_name in doc.file_names {
        let result = remove_file(format!("./uploads/{}", doc_name)).await;
        match result {
            Ok(_) => {}
            Err(_) => { println!("Файл {} відсутній", doc_name) }
        }
    }

    collection.delete_one(filter, None).await.expect("Не вийшло видалити документ");


    Ok(HttpResponse::Ok().body("Files deleted successfully"))
}

