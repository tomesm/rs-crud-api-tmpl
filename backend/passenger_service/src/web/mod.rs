#![allow(dead_code)]
#![allow(unused)]

use crate::model::{self, Db, Passenger};
use crate::security;
use std::{path::Path, sync::Arc};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

use std::string::String;

use serde_json::json;
use utoipa_swagger_ui::Config;
mod filter_auth;
mod filter_utils;
pub mod handlers;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "X-Auth-Token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("123"))),
        )
    }
}

pub async fn start_web_server(folder: &str, port: u16, db: Arc<Db>) -> Result<(), Error> {
    // validate the web folder
    if !Path::new(folder).exists() {
        return Err(Error::FailStartWebFolderNotFound(folder.to_string()));
    }

    let config = Arc::new(Config::from("/api-doc.json"));
    // Prepare swagger ui
    #[derive(OpenApi)]
    #[openapi(
        paths(handlers::list_passengers), 
        components(schemas(Passenger)),
        modifiers(&SecurityAddon),
        tags(
            (name = "Passengers", description = "Passengers items management API")
        )
    )]
    struct ApiDoc;



    // Swagger JSON API
    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));
    // Swagger UI Endpoints
    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    // // Passengers routes
    let apis = handlers::handlers("api", db);
    // Static content -- index.html and all other files
    let content = warp::fs::dir(folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end()) // = localhost:port/
        .and(warp::fs::file(format!("{}/index.html", folder))); // = localhost:port/index.html
    let static_site = content.or(root_index);

    // Combine all routes
    let routes = api_doc
        .or(swagger_ui)
        .or(apis)
        .or(static_site);

    println!("Start 127.0.0.1:{} at {}", port, folder);
    warp::serve(routes)
    .run(([127, 0, 0, 1], port)).await;

    Ok(())
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static("/swagger-ui/"))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error("Fail authentication missing X-Auth-Token header.")]
    FailAuthMissingXAuth,
}

// region:    Warp Custom Error
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}
impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{}", other))
    }
}
impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{}", other))
    }
}
impl From<security::Error> for warp::Rejection {
    fn from(other: security::Error) -> Self {
        WebErrorMessage::rejection("security::Error", format!("{}", other))
    }
}
// endregion: Warp Custom Error
