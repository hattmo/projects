use crate::openai::Message;
use anyhow::Result;
use mongodb::{
    bson::{doc, to_document, Uuid},
    options::{ClientOptions, ServerApiVersion, UpdateOptions},
    Client,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SessionData {
    pub messages: Vec<Message>,
    #[serde(rename = "_id")]
    id: Uuid,
}

pub async fn create_client() -> Result<Client> {
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    client_options.app_name = Some("aiexec".to_string());
    client_options.server_api = Some(
        mongodb::options::ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build(),
    );
    let client = mongodb::Client::with_options(client_options)?;
    Ok(client)
}

pub async fn get_session_data(client: &Client, session_id: Uuid) -> Result<SessionData> {
    let coll = client.database("aiexec").collection("session_data");
    Ok(coll
        .find_one(Some(doc! {"id":session_id}), None)
        .await?
        .unwrap_or(SessionData {
            messages: vec![],
            id: session_id,
        }))
}
pub async fn put_session_data(client: &Client, session: SessionData) -> Result<()> {
    let coll = client
        .database("aiexec")
        .collection::<SessionData>("session_data");
    let options = UpdateOptions::builder().upsert(true).build();
    let session_id = session.id;
    let session = to_document(&session)?;
    coll.update_one(
        doc! {"_id":session_id},
        doc! {"$set":session},
        Some(options),
    )
    .await
    .unwrap();
    Ok(())
}
