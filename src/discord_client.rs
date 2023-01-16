#![allow(dead_code)]
use crate::model::Activity;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub struct Client {
    pub client_id: String,
    connected: bool,
    socket: Option<File>,
}

fn pack(opcode: u32, data_len: u32) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    for byte_array in &[opcode.to_le_bytes(), data_len.to_le_bytes()] {
        bytes.extend_from_slice(byte_array);
    }

    Ok(bytes)
}

impl Client {
    pub fn new(client_id: &str) -> Result<Self> {
        let client = Self {
            client_id: client_id.to_string(),
            connected: false,
            socket: None,
        };

        Ok(client)
    }

    pub fn connect(&mut self) -> Result<()> {
        self.connect_ipc()?;
        self.send_payload()?;

        self.connected = true;

        Ok(())
    }

    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = PathBuf::from(format!("\\\\?\\pipe\\discord-ipc-{}", i));

            match OpenOptions::new().read(true).write(true).open(&path) {
                Ok(file) => {
                    self.socket = Some(file);
                    return Ok(());
                }
                Err(_) => continue,
            }
        }

        Err("Could not connect to Discord IPC".into())
    }

    fn send(&mut self, data: Value, opcode: u8) -> Result<()> {
        let data_string = data.to_string();
        let header = pack(opcode.into(), data_string.len() as u32)?;

        self.write(&header)?;
        self.write(data_string.as_bytes())?;

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().expect("Client not connected");

        socket.write_all(data)?;

        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

        socket.read_exact(buffer)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {}

        let socket = self.socket.as_mut().unwrap();
        socket.flush()?;

        Ok(())
    }

    fn get_client_id(&self) -> &String {
        &self.client_id
    }

    fn send_payload(&mut self) -> Result<()> {
        self.send(
            json!({
                "v": 1,
                "client_id": self.get_client_id()
            }),
            0,
        )?;

        Ok(())
    }

    pub fn set_activity(&mut self, activity_payload: Activity) -> Result<()> {
        let data = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": activity_payload
            },
            "nonce": Uuid::new_v4().to_string()
        });

        println!("{:?}", data);
        self.send(data, 1)?;

        Ok(())
    }
}
