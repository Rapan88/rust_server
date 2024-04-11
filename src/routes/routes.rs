use actix_web::web;
use crate::handlers::doc_handler::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/check-health")
            .route(web::get().to(check_health))
    );
    cfg.service(
        web::resource("/doc")
            .route(web::post().to(save_files))
            .route(web::get().to(get_docs))
            .route(web::delete().to(delete_doc))
    );
    cfg.service(
        web::resource("/last-doc")
            .route(web::post().to(save_files))
            .route(web::get().to(get_latest_doc))
            .route(web::delete().to(delete_doc))
    );
}