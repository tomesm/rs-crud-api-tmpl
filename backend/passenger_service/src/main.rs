mod model;
mod security;
mod web;

use std::{env, sync::Arc};

use model::init_db;
use web::start_web_server;

const DEFAULT_WEB_FOLDER: &'static str = "web/";
const DEFAULT_WEB_PORT: u16 = 9090;

#[tokio::main]
async fn main() {
    // compute the web folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = args
        .pop()
        .unwrap_or_else(|| DEFAULT_WEB_PORT.to_string())
        .parse::<u16>()
        .unwrap_or(DEFAULT_WEB_PORT);

    // get the database
    // TODO - loop until valit database connection
    let db = init_db().await.expect(" Can not init database.");
    let db = Arc::new(db);

    // start the server
    match start_web_server(&web_folder, web_port, db).await {
        Ok(_) => println!("Server ended."),
        Err(ex) => println!("ERROR - web server failed to start. Cause {:?}", ex),
    }
}
