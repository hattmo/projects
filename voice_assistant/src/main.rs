mod chat;

use anyhow::anyhow;
use axum::{
    extract::MatchedPath, http::Request, response::IntoResponse, routing::*, serve::Listener, Form,
};
use futures::StreamExt;
use ngrok::{
    conn::ConnInfo,
    prelude::TunnelBuilder,
    tunnel::{EndpointInfo, HttpTunnel},
    EndpointConn,
};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, str::FromStr, time::Duration};
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct NgrokProxy {
    tun: HttpTunnel,
}

impl Listener for NgrokProxy {
    type Io = EndpointConn;

    type Addr = SocketAddr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        let mut backoff = 1;
        let conn = loop {
            match self.tun.next().await {
                Some(Ok(conn)) => break conn,
                Some(Err(e)) => tracing::error!(%e, "Error accepting next connection"),
                None => tracing::error!("No more connections in stream, stream closed"),
            }
            tracing::error!("Waiting to try again in {backoff} seconds");
            sleep(Duration::from_secs(backoff)).await;
            backoff = backoff.saturating_mul(2);
        };
        let remote = conn.remote_addr();
        (conn, remote)
    }

    fn local_addr(&self) -> tokio::io::Result<Self::Addr> {
        SocketAddr::from_str("0.0.0.0:0").map_err(tokio::io::Error::other)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tun = ngrok::Session::builder()
        .authtoken(std::env::var("NGROK_API_KEY").expect("Missing NGROK_API_KEY"))
        .connect()
        .await?
        .http_endpoint()
        .listen()
        .await?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "voice_assistant=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/converse", post(converse))
        .route("/timeout", post(timeout))
        .route("/status", post(status))
        .route("/", post(new_call))
        .route("/", get(|| async { "Im Running!" }))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );
    tracing::info!("Tunnel started on URL: {:?}", tun.url());
    let mut fields = HashMap::new();
    fields.insert("VoiceUrl", tun.url().to_string());
    fields.insert("VoiceMethod", "POST".to_string());
    fields.insert("StatusCallback", format!("{}/status", tun.url()));
    fields.insert("StatusCallbackMethod", "POST".to_string());
    let response = reqwest::Client::new()
        .post(format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers/{}.json",
            std::env::var("TWILIO_ACCOUNT_SID").expect("Missing TWILIO_ACCOUNT_SID"),
            std::env::var("TWILIO_NUMBER_SID").expect("Missing TWILIO_NUMBER_SID")
        ))
        .basic_auth(
            std::env::var("TWILIO_ACCOUNT_SID").expect("Missing TWILIO_ACCOUNT_SID"),
            Some(std::env::var("TWILIO_API_KEY").expect("Missing TWILIO_API_KEY")),
        )
        .form(&fields)
        .send()
        .await?;
    tracing::info!(?response, "Sent twilio request");
    if response.status().is_success() {
        tracing::info!("Callback configured successfully!")
    } else {
        return Err(anyhow!("Failed to configure twilio"));
    }
    tracing::info!("Starting server");

    axum::serve::serve(NgrokProxy { tun }, app).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct RequestData {
    #[serde(rename = "CallSid")]
    call_sid: String,
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "CallStatus")]
    call_status: String,
    #[serde(rename = "SpeechResult")]
    speech_result: Option<String>,
}

async fn new_call(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    util::respond("Hello, how may I help you?")
}

async fn converse(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    if let Some(input) = data.speech_result {
        let output = global::chat(data.call_sid, input).await;
        if let Some(mut output) = output {
            output += "\n\nDo you need additional assistance?";
            return util::respond(&output);
        } else {
            return util::respond("Something went wrong. Please try again.");
        }
    };
    util::respond("I didn't get that. Please try again.")
}

async fn timeout(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    util::respond("Are you still there?")
}

async fn status(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("Status change: {}", data.call_status);
    if data.from != "+17143236041" {
        return ();
    }
    if data.call_status == "completed" {
        global::clear_chat(data.call_sid).await;
    }
    ()
}

mod util {
    use axum::http::{header, HeaderName};

    pub fn respond<'a, 'b>(response: &'a str) -> ([(HeaderName, &'b str); 1], String) {
        (
            [(header::CONTENT_TYPE, "text/xml")],
            format!(include_str!("./respond.xml"), response,),
        )
    }

    pub fn reject() -> ([(HeaderName, &'static str); 1], String) {
        (
            [(header::CONTENT_TYPE, "text/xml")],
            format!(include_str!("./reject.xml")),
        )
    }
}

mod global {
    use crate::chat::Chats;
    use tokio::sync::OnceCell;

    static CHATS: OnceCell<Chats> = OnceCell::const_new();
    pub async fn chat(id: String, request: String) -> Option<String> {
        CHATS
            .get_or_init(|| async { Chats::new() })
            .await
            .chat(id, request)
            .await
    }

    pub async fn clear_chat(id: String) {
        CHATS
            .get_or_init(|| async { Chats::new() })
            .await
            .clear_chat(id)
            .await;
    }
}
