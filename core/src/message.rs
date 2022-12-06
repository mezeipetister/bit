use crate::{
    context::{Context, VERSION},
    prelude::{BitError, BitResult},
};
use chrono::Utc;
use proto::bit_sync::PacketBytes;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::Streaming;

pub enum Status {
    Ok = 0,
    UnAuthorized = 1,
    VersionError = 2,
    BehindRemote = 3,
    Internal = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Header { key: String, value: String },
    Data(Vec<u8>),
}

impl Packet {
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    pub fn as_packet_bytes(&self) -> PacketBytes {
        PacketBytes {
            pkt_data: self.as_bytes(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Message {
    header_stream: BTreeMap<String, String>,
    body_stream: Vec<Vec<u8>>,
}

impl Message {
    pub fn new() -> Self {
        Message::default().set_bit_version().set_dtime()
    }
    pub fn new_response(status: Status) -> Self {
        Message::new().set_status(status)
    }
    pub fn new_request(path: &str) -> Self {
        Message::new().set_path(path)
    }
    pub async fn from_packet_stream(mut pkt_stream: Streaming<PacketBytes>) -> BitResult<Self> {
        let mut res: Message = Message::default();
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
    pub fn get_header_map(&self) -> &BTreeMap<String, String> {
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
    pub fn set_path(mut self, path: &str) -> Self {
        self.add_header("path", path)
    }
    pub fn set_status(mut self, status: Status) -> Self {
        self.add_header("status", status as i32)
    }
    fn set_bit_version(mut self) -> Self {
        self.add_header("bit_version", VERSION)
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
    pub fn set_body<T>(mut self, body: Vec<T>) -> BitResult<Self>
    where
        T: Serialize,
    {
        let mut _body: Vec<Vec<u8>> = Vec::new();
        for item in body {
            _body.push(bincode::serialize(&item)?);
        }
        self.body_stream = _body;
        Ok(self)
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

pub trait ToMessage {
    fn to_message(self) -> Message;
}
