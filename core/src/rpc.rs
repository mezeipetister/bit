use std::net::SocketAddr;

use proto::bit_sync::{bit_sync_client::BitSyncClient, bit_sync_server::*, PacketBytes};
use tokio::sync::oneshot::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    transport::{server::Server, Channel},
    Request, Response, Streaming,
};

use crate::{
    context::Context,
    message::{Message, ToMessage},
    prelude::{BitError, BitResult},
};

pub struct RpcClient {
    channel: BitSyncClient<Channel>,
}

impl RpcClient {
    pub async fn new(ctx: &Context) -> BitResult<Self> {
        let r = Self {
            channel: BitSyncClient::connect(
                ctx.remote_address()
                    .ok_or(BitError::new("No remote added"))?
                    .to_owned(),
            )
            .await?,
        };
        Ok(r)
    }
    pub async fn send(&mut self, message: Message) -> BitResult<Message> {
        let req: Vec<PacketBytes> = message
            .into_packet_stream()
            .into_iter()
            .map(|p| PacketBytes {
                pkt_data: bincode::serialize(&p).unwrap(),
            })
            .collect();
        let request = Request::new(futures_util::stream::iter(req));
        let res = self.channel.message(request).await?.into_inner();

        let r = Message::from_packet_stream(res).await?;
        Ok(r)
    }
}

pub struct RpcServer {
    socket_address: &'static str,
    context: Context,
}

#[tonic::async_trait]
impl BitSync for RpcServer {
    type MessageStream = ReceiverStream<Result<PacketBytes, tonic::Status>>;

    async fn message(
        &self,
        request: tonic::Request<tonic::Streaming<PacketBytes>>,
    ) -> Result<tonic::Response<Self::MessageStream>, tonic::Status> {
        let r = Message::from_packet_stream(request.into_inner())
            .await
            .unwrap();

        // Create channel for stream response
        let (mut tx, rx) = tokio::sync::mpsc::channel(100);

        let res_msg: Message = Message::new(&self.context);

        // Send the result items through the channel
        tokio::spawn(async move {
            for packet in res_msg.into_packet_stream().into_iter() {
                tx.send(Ok(packet.as_packet_bytes())).await.unwrap();
            }
        });

        // Send back the receiver
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

impl RpcServer {
    pub fn new(ctx: &Context, socket_address: &'static str) -> Self {
        Self {
            context: ctx.to_owned(),
            socket_address,
        }
    }
    pub async fn start(self, rx: Receiver<()>) -> BitResult<()> {
        let addr = self.socket_address.parse().unwrap();
        // Spawn the server into a runtime
        tokio::task::spawn(async move {
            Server::builder()
                .add_service(BitSyncServer::new(self))
                .serve_with_shutdown(addr, async { rx.await.unwrap() })
                .await
        });
        Ok(())
    }
}
