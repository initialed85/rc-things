use std::env;
use std::net::SocketAddr;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn get_socket_addr_from_env() -> SocketAddr {
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let socket_addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    return socket_addr;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessage {
    pub throttle: f32,
    pub brake: f32,
    pub steering: f32,
    pub handbrake: bool,
    pub up: bool,
    pub down: bool,
    pub left_drive: f32,
    pub right_drive: f32,
}

pub fn serialize<T>(t: T) -> Vec<u8>
where
    T: Serialize,
{
    return rmp_serde::to_vec(&t).unwrap();
}

pub fn deserialize<T>(message: Vec<u8>) -> T
where
    T: DeserializeOwned,
{
    return rmp_serde::from_slice::<T>(message.as_slice()).unwrap();
}
