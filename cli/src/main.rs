use bit_core::{context::Context, db::Database, message::Message};
use tokio::sync::oneshot;

mod input_parser;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let params: Vec<_> = std::env::args().collect();

    if params.len() > 1 {
        match params[1].as_str() {
            "server" => {
                let ctx = Context::new_server().unwrap();
                // Create shutdown channel
                let (tx, rx) = oneshot::channel();
                let server = bit_core::rpc::RpcServer::new(&ctx, "[::1]:17017");
                server.start(rx).await.unwrap();
                tokio::signal::ctrl_c().await.unwrap();
                println!("SIGINT");
                // Send shutdown signal after SIGINT received
                let _ = tx.send(());
            }
            "commit" => {
                let ctx = Context::new_client().unwrap();
                let mut client = bit_core::rpc::RpcClient::new(&ctx).await.unwrap();
                let r = client.send(Message::new_request("/commit")).await.unwrap();
                println!("{:?}", r);
            }
            "init" => {
                let _ = Database::load().await.lock().await.init().await;
            }
            _ => (),
        }
    }
    Ok(())
}
