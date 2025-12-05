use futures_util::lock::Mutex;
use notify::{Watcher, event::ModifyKind};
use actix_web::{get, App, HttpRequest, HttpServer, Responder, web};
use actix_files::NamedFile;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use std::{path::PathBuf, time::Duration};

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "index.html".parse().unwrap(); Ok(NamedFile::open(path)?)
}

struct AppState {
    clients: Mutex::<Vec<mpsc::Sender<actix_sse::Event>>>,
}

#[get("/sse")]
async fn sse_handler(app_state: web::Data<AppState>) -> impl Responder {
    println!("SSE Connection received");
    let (tx, rx) = mpsc::channel(10);

    app_state.clients.lock().await.push(tx.clone());

    tokio::time::sleep(Duration::from_secs(2)).await;
    let _ = tx.send(actix_sse::Event::Comment("my comment".into())).await;
    match tx
        .send(actix_sse::Data::new("my data").event("chat_msg").into())
        .await {
            Ok(()) => println!("Message sent"),
            Err(err) => println!("Error sending the message: {:?}", err)
        }

    let event_stream = ReceiverStream::new(rx);
    actix_sse::Sse::from_infallible_stream(event_stream)
        .with_retry_duration(Duration::from_secs(5))
        .with_keep_alive(Duration::from_secs(15))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState { clients: Mutex::new(vec![]) } );

    let web_state = app_state.clone();
    tokio::spawn(async move {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();

        let mut watcher = notify::recommended_watcher(tx).expect("unable to get a watcher");

        watcher.watch(
            std::path::Path::new("./test.txt"),
            notify::RecursiveMode::Recursive
        ).expect("unable to get a watcher");

        for res in rx {
            match res {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Modify(ModifyKind::Data(_)) => {
                            let clients = web_state.get_ref().clients.lock().await;
                            for c_tx in clients.iter(){
                                let _ = c_tx.send(actix_sse::Event::Comment("my comment".into())).await;
                                match c_tx
                                    .send(actix_sse::Data::new("reload").event("file_content_modify").into())
                                .await {
                                    Ok(()) => println!("Message sent from modify"),
                                    Err(err) => println!("Error sending the message: {:#?}", err)
                                }
                            }
                        }
                        _ => {}
                    }
                    // println!("event: {:?}", event);
                },
                Err(err) => println!("watch error: {:?}", err),
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .service(sse_handler)
            .service(actix_files::Files::new("/static", "./public"))
    })
        .bind(("127.0.0.1", 8081))?
        .run()
    .await
}
