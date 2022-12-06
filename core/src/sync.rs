use crate::{
    context::Context,
    prelude::{BitError, BitResult},
};
use chrono::Utc;
use proto::bit_sync::PacketBytes;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::Streaming;

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Header { key: String, value: String },
    Data(Vec<u8>),
}

#[derive(Debug, Default)]
pub struct Request {
    header_stream: BTreeMap<String, String>,
    body_stream: Vec<Vec<u8>>,
}

impl Request {
    pub async fn from_packet_stream(mut pkt_stream: Streaming<PacketBytes>) -> BitResult<Self> {
        let mut res: Request = Request::default();
        while let Some(bytes) = pkt_stream.message().await? {
            let pkt: Packet = bincode::deserialize(&bytes.pkt_data)?;
            res = res.insert_packet(pkt);
        }
        Ok(res)
    }
    fn insert_packet(mut self, packet: Packet) -> Self {
        match packet {
            Packet::Header { key, value } => {
                let _ = self.header_stream.insert(key, value);
            }
            Packet::Data(bytes) => self.body_stream.push(bytes),
        }
        self
    }
    pub fn get_path(&self) -> BitResult<&String> {
        self.get_header("path")
            .ok_or(BitError::new("Request must have path header attr"))
    }
    pub fn get_username(&self) -> BitResult<&String> {
        self.get_header("username")
            .ok_or(BitError::new("Request must have username header attr"))
    }
    pub fn get_access_token(&self) -> BitResult<&String> {
        self.get_header("access_token").ok_or(BitError::new(
            "Unauthorized request. No access token provided",
        ))
    }
    pub fn get_header(&self, k: &str) -> Option<&String> {
        self.header_stream.get(k)
    }
    pub fn get_header_stream(&self) -> &BTreeMap<String, String> {
        &self.header_stream
    }
    pub fn get_body_stream_raw(self) -> Vec<Vec<u8>> {
        unimplemented!()
    }
    pub fn get_body_stream<T>(self) -> BitResult<Vec<T>>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut res: Vec<T> = Vec::new();
        for item in self.body_stream {
            res.push(bincode::deserialize(&item)?);
        }
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Response<T>
where
    T: Serialize,
{
    header_stream: HashMap<String, String>,
    body_stream: Vec<T>,
}

impl<T> Response<T>
where
    T: Serialize,
    Self: Sized,
{
    pub fn new(ctx: &Context) -> Self {
        let mut r = Response {
            header_stream: HashMap::new(),
            body_stream: Vec::new(),
        };
        // Auto set bit version and dtime header attrs
        r.set_bit_version(ctx).set_dtime()
    }
    fn set_bit_version(mut self, ctx: &Context) -> Self {
        self.add_header("bit_version", ctx.bit_version())
    }
    fn set_dtime(mut self) -> Self {
        self.add_header("dtime", Utc::now().to_rfc3339())
    }
    pub fn set_content_type<A>(mut self, v: A) -> Self
    where
        A: ToString,
    {
        self.add_header("content_type", v)
    }
    pub fn set_content_length<A>(mut self, length: A) -> Self
    where
        A: ToString,
    {
        self.add_header("content_length", length)
    }
    pub fn add_header<K, V>(mut self, k: K, v: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.header_stream.insert(k.to_string(), v.to_string());
        self
    }
    pub fn into_packet_stream(self) -> Vec<Packet> {
        let mut res: Vec<Packet> = Vec::new();
        self.header_stream
            .into_iter()
            .for_each(|(k, v)| res.push(Packet::Header { key: k, value: v }));
        self.body_stream
            .into_iter()
            .for_each(|d| match bincode::serialize(&d) {
                Ok(data) => res.push(Packet::Data(data)),
                Err(_) => (),
            });
        res
    }
}

pub trait ToResponse<T>
where
    T: Serialize,
{
    fn to_response(self, ctx: &Context) -> Response<T>;
}
