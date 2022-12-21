use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Ok,
    Error,
    DatabaseNotFound,
    KeyNotExists,
    KeyAlreadyExists,
    SyntaxError,
    InvalidQuery,
    InvalidBody,
    InvalidBson,
    InvalidAuth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: Option<String>,
    pub status: Status,
    pub body: Option<bson::Bson>,
}

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream })
    }

    pub async fn request(
        &mut self,
        query: String,
        database: String,
        auth: Option<String>,
    ) -> tokio::io::Result<Response> {
        let doc = bson::doc! {
            "body": {
                "query": query,
                "database": database,
            },
            "auth": auth,
        };

        self.stream.write_all(&bson::to_vec(&doc).unwrap()).await?;

        let mut buf = [0; 1024];
        let n = self.stream.read(&mut buf).await?;

        let response: Response = bson::from_slice(&buf[..n]).unwrap();

        Ok(response)
    }
}
