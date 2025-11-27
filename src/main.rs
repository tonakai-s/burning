fn main() {
    let server = tiny_http::Server::http("127.0.0.1:8080").unwrap();
    loop {
        match server.recv() {
            Ok(req) => println!("Request: {req:?}"),
            Err(err) => println!("Err: {err:?}")
        }
    }
}
