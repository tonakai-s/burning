use std::sync::Arc;

use actix_web::web;
use burning::{args::Args, server::{self, AppState}, watcher};
use clap::Parser;
use tokio::sync::Mutex;

// TODO: handle termination of the program
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Arc::new(Args::parse());

    let web_state = web::Data::new(AppState { clients: Mutex::new(vec![]) });
    
    watcher::watch(args.clone(), web_state.clone());
    server::build(web_state).await
}
