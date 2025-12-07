use std::sync::Arc;

use actix_web::web;
use notify::{Watcher, event::ModifyKind};

use crate::{args::Args, server::AppState};

pub fn watch(args: Arc<Args>, web_state: web::Data<AppState>) {

    let web_state = web_state.clone();
    tokio::spawn(async move {
        let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();

        let mut watcher = notify::recommended_watcher(tx).expect("unable to get a watcher, stop the program manually");

        // TODO: The whole program need to stop when this kind of error happen
        watcher.watch(
            std::path::Path::new(&args.watch.clone()),
            notify::RecursiveMode::Recursive
        ).expect("unable to get a watcher, stop the program manually");

        println!("the watcher is successfully watching!");
        for res in rx {
            match res {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Modify(ModifyKind::Data(_)) => {
                            let mut clients = web_state.get_ref().clients.lock().await;
                            for i in (0..clients.len()).rev() {
                                match clients[i]
                                    // TODO: is clone really needed?
                                    .send(actix_sse::Data::new(args.target.clone()).event("reload_from_file").into())
                                .await {
                                    Ok(()) => println!("modification notified"),
                                    Err(_) => {
                                        clients.remove(i);
                                        println!("a client was removed since it cannot receive events");
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
                Err(err) => println!("an error ocurred while watching the filesystem: {:?}", err),
            }
        }
    });
}
