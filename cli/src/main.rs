use bit_core::message::Message;
use tokio::sync::oneshot;

mod input_parser;

#[tokio::main]
async fn main() {
    let params: Vec<_> = std::env::args().collect();
    let ctx = bit_core::context::Context::new(bit_core::context::Mode::Server);
    if params.len() > 1 {
        match params[1].as_str() {
            "server" => {
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
                let mut client = bit_core::rpc::RpcClient::new(&ctx).await.unwrap();
                let r = client
                    .send(Message::new_request(&ctx, "/commit"))
                    .await
                    .unwrap();
                println!("{:?}", r);
            }
            _ => (),
        }
    }
}
