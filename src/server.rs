use std::{path::PathBuf, time::Duration};

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, Responder, get, web};
use tokio::sync::{Mutex, mpsc};
use tokio_stream::wrappers::ReceiverStream;

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "index.html".parse().unwrap(); Ok(NamedFile::open(path)?)
}

pub struct AppState {
    pub clients: Mutex::<Vec<mpsc::Sender<actix_sse::Event>>>,
}

#[get("/sse")]
async fn sse_handler(app_state: web::Data<AppState>) -> impl Responder {
    println!("SSE Connection received");
    let (tx, rx) = mpsc::channel(10);

    app_state.clients.lock().await.push(tx.clone());
    println!("clients count: {:?}", app_state.clients.lock().await.len());

    tokio::time::sleep(Duration::from_secs(2)).await;
    // let _ = tx.send(actix_sse::Event::Comment("my comment".into())).await;
    match tx
        .send(actix_sse::Data::new("pong").event("ping").into())
        .await {
            Ok(()) => println!("ping sent"),
            Err(err) => println!("error sending a start ping: {:?}", err)
        }

    let event_stream = ReceiverStream::new(rx);
    actix_sse::Sse::from_infallible_stream(event_stream)
        .with_retry_duration(Duration::from_secs(5))
        .with_keep_alive(Duration::from_secs(15))
}

pub async fn build(app_state: web::Data<AppState>) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .service(sse_handler)
            .service(actix_files::Files::new("/static", "./public"))
    })
        .bind(("127.0.0.1", 8081))?;
    println!("SSE server up and running");

    server.run()
        .await
}
