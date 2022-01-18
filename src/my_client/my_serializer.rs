use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadBuffer, ReadingTcpContractFail, SocketReader},
    TcpSocketSerializer,
};

static CLCR: &[u8] = &[13u8, 10u8];
pub struct MySerializer {
    read_buffer: ReadBuffer,
}

impl MySerializer {
    pub fn new() -> Self {
        Self {
            read_buffer: ReadBuffer::new(1024 * 24),
        }
    }
}

#[async_trait]
impl TcpSocketSerializer<String> for MySerializer {
    fn serialize(&self, contract: String) -> Vec<u8> {
        let contract = contract.as_bytes();
        let mut result = Vec::with_capacity(contract.len() + CLCR.len());
        result.extend_from_slice(contract);
        result.extend_from_slice(CLCR);

        result
    }

    fn get_ping(&self) -> String {
        "PING".to_string()
    }

    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
    ) -> Result<String, ReadingTcpContractFail> {
        let result = socket_reader
            .read_until_end_marker(&mut self.read_buffer, CLCR)
            .await?;

        let result = String::from_utf8((result[..result.len() - CLCR.len()]).to_vec())?;

        Ok(result)
    }

    fn apply_packet(&mut self, contract: &String) -> bool {
        false
    }
}
