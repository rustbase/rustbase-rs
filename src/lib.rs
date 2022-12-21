use client::Response;
use tokio::net::ToSocketAddrs;

mod client;
pub mod error;

pub struct Query;

impl Query {
    pub fn insert(key: &str, value: serde_json::Value) -> String {
        format!(
            "insert {} in {}",
            serde_json::to_string(&value).unwrap(),
            key
        )
    }

    pub fn get(key: &str) -> String {
        format!("get {}", key)
    }

    pub fn delete(key: &str) -> String {
        format!("delete {}", key)
    }

    pub fn update(key: &str, value: serde_json::Value) -> String {
        format!(
            "update {} in {}",
            serde_json::to_string(&value).unwrap(),
            key
        )
    }
}

#[derive(Debug)]
pub struct Rustbase {
    client: client::Client,
    database: String,
}

impl Rustbase {
    pub async fn connect(
        addr: impl ToSocketAddrs,
        database: String,
    ) -> Result<Self, std::io::Error> {
        let client = client::Client::connect(addr).await?;
        Ok(Self { client, database })
    }

    pub fn set_database(&mut self, database: String) {
        self.database = database;
    }

    fn parse_response(response: Response) -> error::Result<Response> {
        match response.status {
            client::Status::Ok => Ok(response),

            _ => Err(error::Error {
                message: response.message,
                status: response.status,
            }),
        }
    }

    pub async fn insert(&mut self, key: &str, value: serde_json::Value) -> error::Result<Response> {
        let query = Query::insert(key, value);

        let response = self
            .client
            .request(query, self.database.clone(), None)
            .await
            .unwrap();

        Self::parse_response(response)
    }

    pub async fn get(&mut self, key: &str) -> error::Result<Response> {
        let query = Query::get(key);

        let response = self
            .client
            .request(query, self.database.clone(), None)
            .await
            .unwrap();

        Self::parse_response(response)
    }

    pub async fn delete(&mut self, key: &str) -> error::Result<Response> {
        let query = Query::delete(key);

        let response = self
            .client
            .request(query, self.database.clone(), None)
            .await
            .unwrap();

        Self::parse_response(response)
    }

    pub async fn update(&mut self, key: &str, value: serde_json::Value) -> error::Result<Response> {
        let query = Query::update(key, value);

        let response = self
            .client
            .request(query, self.database.clone(), None)
            .await
            .unwrap();

        Self::parse_response(response)
    }
}
