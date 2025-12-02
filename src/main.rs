use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

//fn main () {
    //let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    //for stream in server.incoming() {
    //    spawn (move || {
    //        let mut websocket = accept(stream.unwrap()).unwrap();
    //        println!("socket connected");
    //        loop {
    //            let msg = websocket.read().unwrap();

    //            if msg.is_binary() || msg.is_text() {
    //                websocket.send(msg).unwrap();
    //            }
    //        }
    //    });
    //}
//}
use actix_web::{web, App, HttpServer, HttpRequest};
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index)).service(actix_files::Files::new("/static", "./public"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
    .await
}
