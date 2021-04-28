use std::collections::HashMap;
use std::rc::Rc;

use async_std::net::{ToSocketAddrs, UdpSocket};

use super::error::*;
use super::message::*;

pub struct Client {
    socket: Rc<UdpSocket>,
}

impl Client {
    pub async fn new<A: ToSocketAddrs>(local_bind_addr: A) -> Result<Client, StunClientError> {
        let socket = UdpSocket::bind(local_bind_addr)
            .await
            .map_err(|e| StunClientError::IOError(e))?;
        Ok(Client {
            socket: Rc::new(socket),
        })
    }

    pub fn from_socket(socket: Rc<UdpSocket>) -> Client {
        Client { socket: socket }
    }

    pub async fn binding_request<A: ToSocketAddrs>(
        &self,
        stun_addr: A,
        attrs: HashMap<u16, Vec<u8>>,
    ) -> Result<Message, StunClientError> {
        let msg = Message::new(METHOD_BINDING, CLASS_REQUEST, attrs);
        let raw_msg = msg.to_raw();
        self.socket
            .send_to(&raw_msg, stun_addr)
            .await
            .map_err(|e| StunClientError::IOError(e))?;

        // Todo: buf.length() < n
        let mut buf = vec![0u8; 1024];
        let (n, _) = self
            .socket
            .recv_from(&mut buf)
            .await
            .map_err(|e| StunClientError::IOError(e))?;

        Message::from_raw(&buf[..n])
    }
}
